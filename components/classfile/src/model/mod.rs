use std::convert::TryFrom;
use std::fmt;

use bitflags::bitflags;

use constants::{ConstantIndex, ConstantPool};

use crate::error::{JvmParseError, JvmParseResult};
use crate::model::attributes::{Code, ConstantValue};

pub mod attributes;
pub mod constants;

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
    Code(Code),
    ConstantValue(ConstantValue),
    SourceFile(ConstantIndex), // MUST be index to Utf8
    Unknown { name: ConstantIndex, value: Vec<u8> },
}
