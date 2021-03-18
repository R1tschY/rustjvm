use crate::error::JvmParseResult;
use crate::model::attributes::{Code, ConstantValue, ExceptionTableEntry};
use crate::model::constants::ConstantPool;
use crate::parse::{parse_bytes_u32, ClassFileEntry, ReadClassFileExt};
use rustjvm_opcode::disasm;
use std::io::Read;

impl ClassFileEntry for ConstantValue {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Self> {
        Ok(ConstantValue {
            constantvalue_index: reader.parse(cpool)?,
        })
    }
}

impl ClassFileEntry for ExceptionTableEntry {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Self> {
        Ok(ExceptionTableEntry {
            start_pc: reader.parse(cpool)?,
            end_pc: reader.parse(cpool)?,
            handler_pc: reader.parse(cpool)?,
            catch_type: reader.parse(cpool)?,
        })
    }
}

impl ClassFileEntry for Code {
    fn parse<T: Read>(reader: &mut T, cpool: &ConstantPool) -> JvmParseResult<Self> {
        Ok(Code {
            max_stack: reader.parse(cpool)?,
            max_locals: reader.parse(cpool)?,
            code: disasm(&parse_bytes_u32(reader)?)?,
            exception_table: reader.parse(cpool)?,
            attributes: reader.parse(cpool)?,
        })
    }
}
