use crate::error::{JvmParseError, JvmParseResult};
use crate::AccessFlags;
use bitflags::_core::convert::TryFrom;
use bitflags::_core::fmt;

pub struct ClassFile {
    pub(crate) magic: u32,
    pub(crate) minor_version: u16,
    pub(crate) major_version: u16,
    pub(crate) constants: ConstantPool,
    pub(crate) access_flags: AccessFlags,
    pub(crate) this_class: ConstantIndex,
    pub(crate) super_class: ConstantIndex,
    pub(crate) interfaces: Vec<ConstantIndex>,
    pub(crate) fields: Vec<Field>,
    pub(crate) methods: Vec<Method>,
    pub(crate) attributes: Vec<Attribute>,
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

    pub fn access_flags(&self) -> AccessFlags {
        self.access_flags.clone()
    }

    pub fn constant_pool(&self) -> &ConstantPool {
        &self.constants
    }

    pub fn this_class(&self) -> ConstantIndex {
        self.this_class
    }

    pub fn super_class(&self) -> ConstantIndex {
        self.super_class
    }

    pub fn interfaces(&self) -> &[ConstantIndex] {
        &self.interfaces
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

pub struct ConstantPool(Vec<Constant>);

impl ConstantPool {
    pub fn new(constant_pool: Vec<Constant>) -> Self {
        Self(constant_pool)
    }

    pub fn all(&self) -> Vec<(ConstantIndex, &Constant)> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_valid())
            .map(|(i, constant)| (ConstantIndex(i as u16), constant))
            .collect()
    }

    pub fn get(&self, index: ConstantIndex) -> Option<&Constant> {
        if index.0 == 0 {
            None
        } else {
            // TODO: check for invalid constant
            self.0.get((index.0 - 1) as usize)
        }
    }

    pub fn resolve_utf8(&self, index: ConstantIndex) -> JvmParseResult<&str> {
        match self.get(index) {
            Some(Constant::Utf8(utf8)) => Ok(utf8),
            Some(_) => Err(JvmParseError::WrongConstantType(
                index,
                "expected Utf8".into(),
            )),
            None => Err(JvmParseError::MissingConstant(index)),
        }
    }
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConstantIndex(pub u16);

impl fmt::Debug for ConstantIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("#{}", self.0))
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    String(ConstantIndex),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name_index: ConstantIndex,
        descriptor_index: ConstantIndex,
    },
    Utf8(String),
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

impl Constant {
    pub fn is_valid(&self) -> bool {
        *self != Constant::InvalidConstant
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

pub struct Field {
    pub access_flags: AccessFlags,
    pub name_index: ConstantIndex,
    pub descriptor_index: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

pub struct Method {
    pub access_flags: AccessFlags,
    pub name_index: ConstantIndex,
    pub descriptor_index: ConstantIndex,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub enum Attribute {
    UnknownAttribute { name: ConstantIndex, value: Vec<u8> },
}
