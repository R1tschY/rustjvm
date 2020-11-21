use crate::error::{JvmParseError, JvmParseResult};
use crate::model::{
    AccessFlags, Attribute, ClassFile, Constant, ConstantIndex, ConstantPool, Field, Method,
};
use byteorder::{BigEndian, ReadBytesExt};
use cesu8::from_java_cesu8;
use std::convert::TryInto;
use std::io::Read;

mod attributes;

pub(crate) trait ReadClassFileExt: Read + Sized {
    fn parse<T: ClassFileEntry>(&mut self, cpool: &ConstantPool) -> JvmParseResult<T> {
        T::parse(self, cpool)
    }
}

impl<R: Read + Sized> ReadClassFileExt for R {}

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

    let cpool = ConstantPool::new(parse_constants(&mut reader)?);
    Ok(ClassFile {
        magic,
        minor_version,
        major_version,
        access_flags: reader.parse(&cpool)?,
        this_class: reader.parse(&cpool)?,
        super_class: reader.parse(&cpool)?,
        interfaces: reader.parse(&cpool)?,
        fields: reader.parse(&cpool)?,
        methods: reader.parse(&cpool)?,
        attributes: reader.parse(&cpool)?,
        constants: cpool,
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
                name_index: ConstantIndex::parse_primitive(reader)?,
            },
            9 => Constant::Fieldref {
                class_index: ConstantIndex::parse_primitive(reader)?,
                name_and_type_index: ConstantIndex::parse_primitive(reader)?,
            },
            10 => Constant::Methodref {
                class_index: ConstantIndex::parse_primitive(reader)?,
                name_and_type_index: ConstantIndex::parse_primitive(reader)?,
            },
            11 => Constant::InterfaceMethodref {
                class_index: ConstantIndex::parse_primitive(reader)?,
                name_and_type_index: ConstantIndex::parse_primitive(reader)?,
            },
            8 => Constant::String(ConstantIndex::parse_primitive(reader)?),
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
                name_index: ConstantIndex::parse_primitive(reader)?,
                descriptor_index: ConstantIndex::parse_primitive(reader)?,
            },
            15 => Constant::MethodHandle {
                reference_kind: reader.read_u8()?.try_into()?,
                reference_index: ConstantIndex::parse_primitive(reader)?,
            },
            16 => Constant::MethodType {
                descriptor_index: ConstantIndex::parse_primitive(reader)?,
            },
            18 => Constant::InvokeDynamic {
                bootstrap_method_attr_index: ConstantIndex::parse_primitive(reader)?,
                name_and_type_index: ConstantIndex::parse_primitive(reader)?,
            },
            1 => Constant::Utf8(
                from_java_cesu8(&parse_bytes_u16(reader)?)
                    .map_err(|_err| {
                        JvmParseError::InvalidFormat(format!("invalid string for constant {}", i))
                    })?
                    .into(),
            ),
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

pub trait ClassFileEntry: Sized {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Self>;
}

pub trait ClassFilePrimitive: Sized {
    fn parse_primitive<T: Read>(reader: &mut T) -> JvmParseResult<Self>;
}

impl<T: ClassFilePrimitive> ClassFileEntry for T {
    #[inline]
    fn parse<R: Read>(reader: &mut R, _: &ConstantPool) -> JvmParseResult<Self> {
        T::parse_primitive(reader)
    }
}

impl ClassFilePrimitive for u16 {
    fn parse_primitive<R: Read>(reader: &mut R) -> JvmParseResult<u16> {
        Ok(reader.read_u16::<BigEndian>()?)
    }
}

pub fn parse_bytes_u16<R: Read>(reader: &mut R) -> JvmParseResult<Vec<u8>> {
    let length = reader.read_u16::<BigEndian>()?;
    let mut buf: Vec<u8> = vec![0; length as usize];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn parse_bytes_u32<R: Read>(reader: &mut R) -> JvmParseResult<Vec<u8>> {
    let length = reader.read_u32::<BigEndian>()?;
    let mut buf: Vec<u8> = vec![0; length as usize];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}

impl<U: ClassFileEntry> ClassFileEntry for Vec<U> {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Vec<U>> {
        let attributes_count = reader.read_u16::<BigEndian>()? as usize;
        (0..attributes_count)
            .map(|_| U::parse(reader, cpool))
            .collect::<JvmParseResult<Vec<U>>>()
    }
}

impl ClassFilePrimitive for ConstantIndex {
    fn parse_primitive<T: Read>(reader: &mut T) -> JvmParseResult<ConstantIndex> {
        Ok(ConstantIndex(reader.read_u16::<BigEndian>()?))
    }
}

impl ClassFilePrimitive for AccessFlags {
    fn parse_primitive<T: Read>(reader: &mut T) -> JvmParseResult<AccessFlags> {
        Ok(AccessFlags::from_bits_truncate(
            reader.read_u16::<BigEndian>()?,
        ))
    }
}

impl ClassFileEntry for Field {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Field> {
        Ok(Field {
            access_flags: reader.parse(cpool)?,
            name_index: reader.parse(cpool)?,
            descriptor_index: reader.parse(cpool)?,
            attributes: reader.parse(cpool)?,
        })
    }
}

impl ClassFileEntry for Method {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Method> {
        Ok(Method {
            access_flags: reader.parse(cpool)?,
            name_index: reader.parse(cpool)?,
            descriptor_index: reader.parse(cpool)?,
            attributes: reader.parse(cpool)?,
        })
    }
}

impl ClassFileEntry for Attribute {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Attribute> {
        let attribute_name_index = ConstantIndex::parse_primitive(reader)?;
        let name = cpool.resolve_utf8(attribute_name_index)?;
        let info = parse_bytes_u32(reader)?;
        let mut slice: &[u8] = &info;

        Ok(match name {
            "Code" => Attribute::Code(slice.parse(cpool)?),
            "ConstantValue" => Attribute::ConstantValue(slice.parse(cpool)?),
            _ => Attribute::Unknown {
                name: attribute_name_index,
                value: info,
            },
        })
    }
}
