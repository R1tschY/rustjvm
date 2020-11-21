use crate::error::{JvmParseError, JvmParseResult};
use crate::AccessFlags;
use bitflags::_core::convert::TryFrom;
use bitflags::_core::fmt;

pub struct ClassFile {
    pub(crate) magic: u32,
    pub(crate) minor_version: u16,
    pub(crate) major_version: u16,
    pub(crate) constants: Vec<Constant>,
    pub(crate) access_flags: AccessFlags,
    pub(crate) this_class: ConstantIndex,
    pub(crate) super_class: ConstantIndex,
    pub(crate) interfaces: Vec<ConstantIndex>,
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

    pub fn constants(&self) -> &[Constant] {
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

    //

    pub fn resolve_constant(&self, index: ConstantIndex) -> Option<&Constant> {
        if index.0 == 0 {
            None
        } else {
            self.constants.get((index.0 - 1) as usize)
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
