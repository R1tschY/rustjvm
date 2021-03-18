use crate::model::{Attribute, ConstantIndex};
use rustjvm_opcode::Opcode;

#[derive(Debug)]
pub struct ConstantValue {
    pub constantvalue_index: ConstantIndex,
}

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Opcode>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: Vec<Attribute>,
}
