use crate::{ArrayType, Opcode};

#[derive(Debug, Clone)]
pub enum DisasmError {
    UnknownOpcode(u8),
    MissingArgument,
    InvalidArgument,
    InvalidArrayType,
}

struct Disasm<'a> {
    bytes: &'a [u8],
    index: usize,
    opcodes: Vec<Opcode>,
}

fn compose_u16(one: u8, two: u8) -> u16 {
    ((one as u16) << 8u16) | (two as u16)
}

fn compose_u32(one: u8, two: u8, three: u8, four: u8) -> u32 {
    ((one as u32) << 8u16) | ((two as u32) << 8u16) | ((three as u32) << 8u16) | (four as u32)
}

fn parse_array_type(index: u8) -> Result<ArrayType, DisasmError> {
    Ok(match index {
        4 => ArrayType::BOOLEAN,
        5 => ArrayType::CHAR,
        6 => ArrayType::FLOAT,
        7 => ArrayType::DOUBLE,
        8 => ArrayType::BYTE,
        9 => ArrayType::SHORT,
        10 => ArrayType::INT,
        11 => ArrayType::LONG,
        _ => return Err(DisasmError::InvalidArrayType),
    })
}

impl<'a> Disasm<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            index: 0,
            opcodes: vec![],
        }
    }

    fn argument(&mut self) -> Result<u8, DisasmError> {
        self.index += 1;
        if self.index < self.bytes.len() {
            Ok(self.bytes[self.index])
        } else {
            Err(DisasmError::MissingArgument)
        }
    }

    fn argument_u16(&mut self) -> Result<u16, DisasmError> {
        self.index += 2;
        if self.index < self.bytes.len() {
            Ok(compose_u16(
                self.bytes[self.index - 1],
                self.bytes[self.index],
            ))
        } else {
            Err(DisasmError::MissingArgument)
        }
    }

    fn argument_u32(&mut self) -> Result<u32, DisasmError> {
        self.index += 4;
        if self.index < self.bytes.len() {
            Ok(compose_u32(
                self.bytes[self.index - 3],
                self.bytes[self.index - 2],
                self.bytes[self.index - 1],
                self.bytes[self.index],
            ))
        } else {
            Err(DisasmError::MissingArgument)
        }
    }

    pub fn process(&mut self) -> Result<(), DisasmError> {
        while self.index < self.bytes.len() {
            let opcode = match self.bytes[self.index] {
                0x32 => Opcode::Aaload,
                0x53 => Opcode::Aastore,
                0x01 => Opcode::AconstNull,
                0x19 => Opcode::Aload(self.argument()?),
                0x2a => Opcode::Aload0,
                0x2b => Opcode::Aload0,
                0x2c => Opcode::Aload2,
                0x2d => Opcode::Aload3,
                0xbd => Opcode::Anewarray(self.argument_u16()?),
                0xb0 => Opcode::Areturn,
                0xbe => Opcode::Arraylength,
                0x3a => Opcode::Astore(self.argument()?),
                0x4b => Opcode::Astore0,
                0x4c => Opcode::Astore1,
                0x4d => Opcode::Astore2,
                0x4e => Opcode::Astore3,
                0xbf => Opcode::Athrow,
                0x33 => Opcode::Baload,
                0x54 => Opcode::Bastore,
                0x10 => Opcode::Bipush(self.argument()? as i8),
                0x34 => Opcode::Caload,
                0x55 => Opcode::Castore,
                0xc0 => Opcode::Checkcast(self.argument_u16()?),
                0x90 => Opcode::D2f,
                0x8e => Opcode::D2i,
                0x8f => Opcode::D2l,
                0x63 => Opcode::Dadd,
                0x31 => Opcode::Daload,
                0x52 => Opcode::Dastore,
                0x98 => Opcode::Dcmpg,
                0x97 => Opcode::Dcmpl,
                0x0e => Opcode::Dconst0,
                0x0f => Opcode::Dconst1,
                0x6f => Opcode::Ddiv,
                0x18 => Opcode::Dload(self.argument()?),
                0x26 => Opcode::Dload0,
                0x27 => Opcode::Dload1,
                0x28 => Opcode::Dload2,
                0x29 => Opcode::Dload3,
                0x6b => Opcode::Dmul,
                0x77 => Opcode::Dneg,
                0x73 => Opcode::Drem,
                0xaf => Opcode::Dreturn,
                0x39 => Opcode::Dstore(self.argument()?),
                0x47 => Opcode::Dstore0,
                0x48 => Opcode::Dstore1,
                0x49 => Opcode::Dstore2,
                0x4a => Opcode::Dstore3,
                0x67 => Opcode::Dsub,
                0x59 => Opcode::Dup,
                0x5a => Opcode::DupX1,
                0x5b => Opcode::DupX2,
                0x5c => Opcode::Dup2,
                0x5d => Opcode::Dup2X1,
                0x5e => Opcode::Dup2X2,
                0x8d => Opcode::F2d,
                0x8b => Opcode::F2i,
                0x8c => Opcode::F2l,
                0x62 => Opcode::Fadd,
                0x30 => Opcode::Faload,
                0x51 => Opcode::Fastore,
                0x96 => Opcode::Fcmpg,
                0x95 => Opcode::Fcmpl,
                0x0b => Opcode::Fconst0,
                0x0c => Opcode::Fconst1,
                0x0d => Opcode::Fconst2,
                0x6e => Opcode::Fdiv,
                0x17 => Opcode::Fload(self.argument()?),
                0x22 => Opcode::Fload0,
                0x23 => Opcode::Fload1,
                0x24 => Opcode::Fload2,
                0x25 => Opcode::Fload3,
                0x6a => Opcode::Fmul,
                0x76 => Opcode::Fneg,
                0x72 => Opcode::Frem,
                0xae => Opcode::Freturn,
                0x38 => Opcode::Fstore(self.argument()?),
                0x43 => Opcode::Fstore0,
                0x44 => Opcode::Fstore1,
                0x45 => Opcode::Fstore2,
                0x46 => Opcode::Fstore3,
                0x66 => Opcode::Fsub,
                0xb4 => Opcode::Getfield(self.argument_u16()?),
                0xa7 => Opcode::Goto(self.argument_u16()?),
                0xc8 => Opcode::GotoW(self.argument_u32()?),
                0x91 => Opcode::I2b,
                0x92 => Opcode::I2c,
                0x87 => Opcode::I2d,
                0x86 => Opcode::I2f,
                0x85 => Opcode::I2l,
                0x93 => Opcode::I2s,
                0x60 => Opcode::Iadd,
                0x2e => Opcode::Iaload,
                0x7e => Opcode::Iand,
                0x4f => Opcode::Iastore,
                0x02 => Opcode::IconstM1,
                0x03 => Opcode::Iconst0,
                0x04 => Opcode::Iconst1,
                0x05 => Opcode::Iconst2,
                0x06 => Opcode::Iconst3,
                0x07 => Opcode::Iconst4,
                0x08 => Opcode::Iconst5,
                0x6c => Opcode::Idiv,
                0xa5 => Opcode::IfAcmpeq(self.argument_u16()?),
                0xa6 => Opcode::IfAcmpne(self.argument_u16()?),
                0x9f => Opcode::IfIcmpeq(self.argument_u16()?),
                0xa0 => Opcode::IfIcmpne(self.argument_u16()?),
                0xa1 => Opcode::IfIcmplt(self.argument_u16()?),
                0xa2 => Opcode::IfIcmpge(self.argument_u16()?),
                0xa3 => Opcode::IfIcmpgt(self.argument_u16()?),
                0xa4 => Opcode::IfIcmple(self.argument_u16()?),
                0x99 => Opcode::Ifeq(self.argument_u16()?),
                0x9a => Opcode::Ifne(self.argument_u16()?),
                0x9b => Opcode::Iflt(self.argument_u16()?),
                0x9c => Opcode::Ifge(self.argument_u16()?),
                0x9d => Opcode::Ifgt(self.argument_u16()?),
                0x9e => Opcode::Ifle(self.argument_u16()?),
                0xc7 => Opcode::Ifnonnull(self.argument_u16()?),
                0xc6 => Opcode::Ifnull(self.argument_u16()?),
                0x84 => Opcode::Iinc(self.argument()?, self.argument()?),
                0x15 => Opcode::Iload(self.argument()?),
                0x1a => Opcode::Iload0,
                0x1b => Opcode::Iload1,
                0x1c => Opcode::Iload2,
                0x1d => Opcode::Iload3,
                0x68 => Opcode::Imul,
                0x74 => Opcode::Ineg,
                0xba => {
                    let opcode = Opcode::Invokedynamic(self.argument_u16()?);
                    if self.argument_u16()? != 0 {
                        return Err(DisasmError::InvalidArgument);
                    }
                    opcode
                }
                0xb9 => {
                    let opcode = Opcode::Invokeinterface(self.argument_u16()?, self.argument()?);
                    if self.argument()? != 0 {
                        return Err(DisasmError::InvalidArgument);
                    }
                    opcode
                }
                0xb7 => Opcode::Invokespecial(self.argument_u16()?),
                0xb8 => Opcode::Invokestatic(self.argument_u16()?),
                0xb6 => Opcode::Invokevirtual(self.argument_u16()?),
                0x80 => Opcode::Ior,
                0x70 => Opcode::Irem,
                0xac => Opcode::Ireturn,
                0x78 => Opcode::Ishl,
                0x7a => Opcode::Ishr,
                0x36 => Opcode::Istore(self.argument()?),
                0x3b => Opcode::Istore0,
                0x3c => Opcode::Istore1,
                0x3d => Opcode::Istore2,
                0x3e => Opcode::Istore3,
                0x64 => Opcode::Isub,
                0x7c => Opcode::Iushr,
                0x82 => Opcode::Ixor,
                0xa8 => Opcode::Jsr(self.argument_u16()?),
                0xc9 => Opcode::JsrW(self.argument_u32()?),
                0x8a => Opcode::L2d,
                0x89 => Opcode::L2f,
                0x88 => Opcode::L2i,
                0x61 => Opcode::Ladd,
                0x2f => Opcode::Laload,
                0x7f => Opcode::Land,
                0x50 => Opcode::Lastore,
                0x94 => Opcode::Lcmp,
                0x09 => Opcode::Lconst0,
                0x0a => Opcode::Lconst1,
                0x12 => Opcode::Ldc(self.argument()?),
                0x13 => Opcode::LdcW(self.argument_u16()?),
                0x14 => Opcode::Ldc2W(self.argument_u16()?),
                0x6d => Opcode::Ldiv,
                0x16 => Opcode::Lload(self.argument()?),
                0x1e => Opcode::Lload0,
                0x1f => Opcode::Lload1,
                0x20 => Opcode::Lload2,
                0x21 => Opcode::Lload3,
                0x69 => Opcode::Lmul,
                0x75 => Opcode::Lneg,
                0xab => unimplemented!(), // Opcode::Lookupswitch(),
                0x81 => Opcode::Lor,
                0x71 => Opcode::Lrem,
                0xad => Opcode::Lreturn,
                0x79 => Opcode::Lshl,
                0x7b => Opcode::Lshr,
                0x37 => Opcode::Lstore(self.argument()?),
                0x3f => Opcode::Lstore0,
                0x40 => Opcode::Lstore1,
                0x41 => Opcode::Lstore2,
                0x42 => Opcode::Lstore3,
                0x65 => Opcode::Lsub,
                0x7d => Opcode::Lushr,
                0x83 => Opcode::Lxor,
                0xc2 => Opcode::Monitorenter,
                0xc3 => Opcode::Monitorexit,
                0xbb => Opcode::New(self.argument_u16()?),
                0xbc => Opcode::Newarray(parse_array_type(self.argument()?)?),
                0x00 => Opcode::Nop,
                0x57 => Opcode::Pop,
                0x58 => Opcode::Pop2,
                0xb5 => Opcode::Putfield(self.argument_u16()?),
                0xb3 => Opcode::Ret(self.argument()?),
                0xb1 => Opcode::Return,
                0x35 => Opcode::Saload,
                0x56 => Opcode::Sastore,
                0x11 => Opcode::Sipush(self.argument_u16()? as i16),
                0xaa => unimplemented!(), //Opcode::Tableswitch(),
                0xc4 => unimplemented!(), // Opcode::Wide(),

                tag => return Err(DisasmError::UnknownOpcode(tag)),
            };
            self.opcodes.push(opcode);
            self.index += 1;
        }
        Ok(())
    }
}

pub fn disasm(bytes: &[u8]) -> Result<Vec<Opcode>, DisasmError> {
    let mut disasm = Disasm::new(bytes);
    disasm.process();
    Ok(disasm.opcodes)
}
