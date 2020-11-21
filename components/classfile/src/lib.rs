use crate::model::{Attribute, ConstantPool, Field};
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt};
use cesu8::from_java_cesu8;
use error::{JvmParseError, JvmParseResult};
use model::{ClassFile, Constant, ConstantIndex};
use std::convert::TryInto;
use std::io;
use std::io::Read;

pub mod error;
pub mod model;

bitflags! {
    pub struct AccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;

        /// Declared final; no subclasses allowed.
        const FINAL = 0x0010;

        /// Treat superclass methods specially when invoked by the invokespecial instruction.
        const SUPER = 0x0020;

        /// Is an interface, not a class.
        const INTERFACE = 0x0200;

        /// Declared abstract; must not be instantiated.
        const ABSTRACT = 0x0400;

        /// Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;

        /// Declared as an annotation type.
        const ANNOTATION = 0x2000;

        /// Declared as an enum type.
        const ENUM = 0x4000;

        /// Is a module, not a class or interface.
        const MODULE = 0x8000;
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
pub fn parse_class_file<T: Read>(mut reader: T) -> JvmParseResult<ClassFile> {
    let magic: u32 = reader.read_u32::<BigEndian>()?;
    if magic != 0xCAFEBABE {
        return Err(JvmParseError::InvalidFormat("invalid magic".into()));
    }

    let minor_version: u16 = reader.read_u16::<BigEndian>()?;
    let major_version: u16 = reader.read_u16::<BigEndian>()?;
    if major_version < 45 {
        return Err(JvmParseError::InvalidFormat(format!(
            "version must be at least 45.0, but got {}.{}",
            major_version, minor_version
        )));
    }

    let constants = ConstantPool::new(parse_constants(&mut reader)?);

    let access_flags = AccessFlags::from_bits_truncate(reader.read_u16::<BigEndian>()?);

    let this_class = ConstantIndex(reader.read_u16::<BigEndian>()?);
    let super_class = ConstantIndex(reader.read_u16::<BigEndian>()?);

    let interfaces_count = reader.read_u16::<BigEndian>()? as usize;
    let interfaces = (0..interfaces_count)
        .map(|_| reader.read_u16::<BigEndian>().map(|c| ConstantIndex(c)))
        .collect::<io::Result<Vec<ConstantIndex>>>()?;

    let fields_count = reader.read_u16::<BigEndian>()? as usize;
    let fields = (0..fields_count)
        .map(|_| parse_field(&mut reader, &constants))
        .collect::<JvmParseResult<Vec<Field>>>()?;

    Ok(ClassFile {
        magic,
        minor_version,
        major_version,
        constants,
        access_flags,
        this_class,
        super_class,
        interfaces,
        fields,
    })
}

fn parse_constants<T: Read>(reader: &mut T) -> JvmParseResult<Vec<Constant>> {
    let constant_pool_count: u16 = reader.read_u16::<BigEndian>()?;
    let mut constants = vec![];
    constants.reserve(constant_pool_count as usize - 1);

    let mut i = 1;
    while i < constant_pool_count {
        let mut long_constant = false;
        let tag = reader.read_u8()?;
        let constant: Constant = match tag {
            7 => Constant::Class {
                name_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            9 => Constant::Fieldref {
                class_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
                name_and_type_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            10 => Constant::Methodref {
                class_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
                name_and_type_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            11 => Constant::InterfaceMethodref {
                class_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
                name_and_type_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            8 => Constant::String(ConstantIndex(reader.read_u16::<BigEndian>()?)),
            3 => Constant::Integer(reader.read_i32::<BigEndian>()?),
            4 => Constant::Float(reader.read_f32::<BigEndian>()?),
            5 => {
                long_constant = true;
                Constant::Long(reader.read_i64::<BigEndian>()?)
            }
            6 => {
                long_constant = true;
                Constant::Double(reader.read_f64::<BigEndian>()?)
            }
            12 => Constant::NameAndType {
                name_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
                descriptor_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            1 => {
                let length = reader.read_u16::<BigEndian>()?;
                let mut buf: Vec<u8> = vec![0; length as usize];
                reader.read_exact(&mut buf)?;
                Constant::Utf8(
                    from_java_cesu8(&buf)
                        .map_err(|_err| {
                            JvmParseError::InvalidFormat(format!(
                                "invalid string for constant {}",
                                i
                            ))
                        })?
                        .into(),
                )
            }
            15 => Constant::MethodHandle {
                reference_kind: reader.read_u8()?.try_into()?,
                reference_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            16 => Constant::MethodType {
                descriptor_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            18 => Constant::InvokeDynamic {
                bootstrap_method_attr_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
                name_and_type_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            _ => {
                return Err(JvmParseError::InvalidFormat(format!(
                    "unknown constant pool tag at {}: {}",
                    i, tag
                )))
            }
        };
        constants.push(constant);
        if long_constant {
            if i + 1 > constant_pool_count {
                return Err(JvmParseError::InvalidFormat(format!(
                    "long/double constant is missing second constant pool index {}",
                    i + 1
                )));
            }

            i += 2;
            constants.push(Constant::InvalidConstant);
        } else {
            i += 1;
        }
    }
    Ok(constants)
}

fn parse_field<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Field> {
    let access_flags = AccessFlags::from_bits_truncate(reader.read_u16::<BigEndian>()?);
    let name_index = ConstantIndex(reader.read_u16::<BigEndian>()?);
    let descriptor_index = ConstantIndex(reader.read_u16::<BigEndian>()?);
    let attributes = parse_attributes(reader, cpool)?;

    Ok(Field {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    })
}

fn parse_attributes<T: Read>(
    reader: &mut T,
    cpool: &ConstantPool,
) -> JvmParseResult<Vec<Attribute>> {
    let attributes_count = reader.read_u16::<BigEndian>()? as usize;
    (0..attributes_count)
        .map(|_| parse_attribute(reader, cpool))
        .collect::<JvmParseResult<Vec<Attribute>>>()
}

fn parse_attribute<T: Read>(reader: &mut T, _cpool: &ConstantPool) -> JvmParseResult<Attribute> {
    let attribute_name_index = ConstantIndex(reader.read_u16::<BigEndian>()?);
    let attribute_length = reader.read_u32::<BigEndian>()?;
    let mut info: Vec<u8> = vec![0; attribute_length as usize];
    reader.read_exact(&mut info)?;

    //let name = cpool.resolve_utf8(attribute_name_index)?;

    Ok(Attribute::UnknownAttribute {
        name: attribute_name_index,
        value: info,
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
