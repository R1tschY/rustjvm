use byteorder::{BigEndian, ReadBytesExt};
use cesu8::{from_java_cesu8, Cesu8DecodingError};
use std::convert::{TryFrom, TryInto};
use std::fmt::Formatter;
use std::io::{Error, Read};
use std::{fmt, io};

#[derive(Debug)]
pub enum JvmParseError {
    Io(io::Error),
    InvalidFormat(String),
}

pub type JvmParseResult<T> = Result<T, JvmParseError>;

impl From<io::Error> for JvmParseError {
    fn from(err: io::Error) -> Self {
        JvmParseError::Io(err)
    }
}

pub struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constants: Vec<Constant>,
}

impl ClassFile {
    pub fn magic(&self) -> u32 {
        self.magic
    }

    pub fn minor_version(&self) -> u16 {
        self.minor_version
    }

    pub fn major_version(&self) -> u16 {
        self.major_version
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }
}

pub struct ConstantPoolInfo {
    tag: u8,
    info: Vec<u8>,
}

pub enum ConstantTag {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18,
}

impl TryFrom<u8> for ConstantTag {
    type Error = JvmParseError;

    fn try_from(value: u8) -> JvmParseResult<Self> {
        Ok(match value {
            7 => ConstantTag::Class,
            9 => ConstantTag::Fieldref,
            10 => ConstantTag::Methodref,
            11 => ConstantTag::InterfaceMethodref,
            8 => ConstantTag::String,
            3 => ConstantTag::Integer,
            4 => ConstantTag::Float,
            5 => ConstantTag::Long,
            6 => ConstantTag::Double,
            12 => ConstantTag::NameAndType,
            1 => ConstantTag::Utf8,
            15 => ConstantTag::MethodHandle,
            16 => ConstantTag::MethodType,
            18 => ConstantTag::InvokeDynamic,
            _ => {
                return Err(JvmParseError::InvalidFormat(format!(
                    "unknown constant pool tag: {}",
                    value
                )))
            }
        })
    }
}

pub struct ConstantIndex(pub u16);

impl fmt::Debug for ConstantIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("#{}", self.0))
    }
}

#[derive(Debug)]
pub enum Constant {
    Class {
        name_index: ConstantIndex,
    },
    Fieldref {
        class_index: ConstantIndex,
        name_and_type_index: ConstantIndex,
    },
    Methodref {
        class_index: ConstantIndex,
        name_and_type_index: ConstantIndex,
    },
    InterfaceMethodref {
        class_index: ConstantIndex,
        name_and_type_index: ConstantIndex,
    },
    String {
        string_index: ConstantIndex,
    },
    Integer {
        value: u32,
    },
    Float {
        value: f32,
    },
    Long {
        value: u64,
    },
    Double {
        value: f64,
    },
    NameAndType {
        name_index: ConstantIndex,
        descriptor_index: ConstantIndex,
    },
    Utf8 {
        value: String,
    },
    MethodHandle {
        reference_kind: ReferenceKind,
        reference_index: ConstantIndex,
    },
    MethodType {
        descriptor_index: ConstantIndex,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: ConstantIndex,
        name_and_type_index: ConstantIndex,
    },
    InvalidConstant,
}

#[derive(Debug)]
pub enum ReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl TryFrom<u8> for ReferenceKind {
    type Error = JvmParseError;

    fn try_from(value: u8) -> JvmParseResult<Self> {
        Ok(match value {
            1 => ReferenceKind::GetField,
            2 => ReferenceKind::GetStatic,
            3 => ReferenceKind::PutField,
            4 => ReferenceKind::PutStatic,
            5 => ReferenceKind::InvokeVirtual,
            6 => ReferenceKind::InvokeStatic,
            7 => ReferenceKind::InvokeSpecial,
            8 => ReferenceKind::NewInvokeSpecial,
            9 => ReferenceKind::InvokeInterface,
            _ => {
                return Err(JvmParseError::InvalidFormat(format!(
                    "unknown reference kind: {}",
                    value
                )))
            }
        })
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html
pub fn parse_class_file<T: Read>(reader: &mut T) -> JvmParseResult<ClassFile> {
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
            8 => Constant::String {
                string_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            3 => Constant::Integer {
                value: reader.read_u32::<BigEndian>()?,
            },
            4 => Constant::Float {
                value: reader.read_f32::<BigEndian>()?,
            },
            5 => Constant::Long {
                value: reader.read_u64::<BigEndian>()?,
            },
            6 => Constant::Double {
                value: reader.read_f64::<BigEndian>()?,
            },
            12 => Constant::NameAndType {
                name_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
                descriptor_index: ConstantIndex(reader.read_u16::<BigEndian>()?),
            },
            1 => {
                let length = reader.read_u16::<BigEndian>()?;
                let mut buf: Vec<u8> = vec![0; length as usize];
                reader.read_exact(&mut buf)?;
                Constant::Utf8 {
                    value: from_java_cesu8(&buf)
                        .map_err(|err| {
                            JvmParseError::InvalidFormat(format!(
                                "invalid string for constant {}",
                                i
                            ))
                        })?
                        .into(),
                }
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

    Ok(ClassFile {
        magic,
        minor_version,
        major_version,
        constants,
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
