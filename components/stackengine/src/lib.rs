use classfile::model::{Constant, ConstantIndex, ConstantPool};
use rustjvm_opcode::Opcode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct JObject {
    fields: Vec<JValue>,
}

pub type JObjectRef = Option<Arc<RefCell<JObject>>>;

#[derive(Clone)]
pub enum JValue {
    Reference(JObjectRef),

    IntArray(Arc<RefCell<[i32]>>),
    LongArray(Arc<RefCell<[i64]>>),
    FloatArray(Arc<RefCell<[f32]>>),
    DoubleArray(Arc<RefCell<[f64]>>),
    BooleanArray(Arc<RefCell<[bool]>>),
    ObjectArray(Arc<RefCell<[JObjectRef]>>),

    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),

    Invalid,
}

impl JValue {
    pub fn as_reference(&self) -> JResult<&JObjectRef> {
        match self {
            JValue::Reference(r) => Ok(r),
            _ => Err(FnError("wrong value type".into())),
        }
    }

    pub fn as_reference_mut(&mut self) -> JResult<&JObjectRef> {
        match self {
            JValue::Reference(r) => Ok(r),
            _ => Err(FnError("wrong value type".into())),
        }
    }

    pub fn as_int(&self) -> JResult<i32> {
        match self {
            JValue::Int(r) => Ok(r.clone()),
            _ => Err(FnError("wrong value type".into())),
        }
    }

    pub fn as_long(&self) -> JResult<i64> {
        match self {
            JValue::Long(r) => Ok(r.clone()),
            _ => Err(FnError("wrong value type".into())),
        }
    }

    pub fn as_float(&self) -> JResult<f32> {
        match self {
            JValue::Float(r) => Ok(r.clone()),
            _ => Err(FnError("wrong value type".into())),
        }
    }

    pub fn as_double(&self) -> JResult<f64> {
        match self {
            JValue::Double(r) => Ok(r.clone()),
            _ => Err(FnError("wrong value type".into())),
        }
    }

    pub fn new_string(s: &str) -> Self {
        unimplemented!()
    }

    pub fn from_constant(index: ConstantIndex, cpool: &ConstantPool) -> JResult<Self> {
        unimplemented!()
        // Ok(match value {
        //     String(index) => match cpool.get(index) {
        //         Constant::Utf8(s) => JValue::new_string(&s),
        //     },
        // })
    }
}

pub struct LoadedMethod {
    code: Vec<Opcode>,
    max_stack: u16,
    max_locals: u16,
    args: u16,
}

pub struct LoadedClass {
    id: String,
    // fields: HashMap<String, Class>,
    methods: HashMap<String, LoadedMethod>,
}

pub struct ClassLoader {
    // classes: HashMap<String, Class>,
}

pub struct JEngine {
    stack: Vec<JValue>,
    locals: Vec<JValue>,
}

impl JEngine {
    fn pop(&mut self) -> JResult<JValue> {
        self.stack
            .pop()
            .ok_or_else(|| FnError("stack underflow".to_string()))
    }

    fn push(&mut self, value: JValue) -> JResult<()> {
        self.stack.push(value);
        Ok(())
    }

    fn pop2(&mut self) -> JResult<(JValue, JValue)> {
        Ok((self.pop()?, self.pop()?))
    }

    fn pop3(&mut self) -> JResult<(JValue, JValue, JValue)> {
        Ok((self.pop()?, self.pop()?, self.pop()?))
    }

    fn top(&self) -> JResult<&JValue> {
        match self.stack.get(self.stack.len() - 1) {
            Some(r) => Ok(r),
            None => Err(FnError("stack underflow".to_string())),
        }
    }

    fn top_mut(&mut self) -> JResult<&mut JValue> {
        let last_index = self.stack.len() - 1;
        match self.stack.get_mut(last_index) {
            Some(r) => Ok(r),
            None => Err(FnError("stack underflow".to_string())),
        }
    }

    fn local(&self, index: u8) -> JResult<&JValue> {
        self.locals
            .get(index as usize)
            .ok_or_else(|| FnError("local does not exist".to_string()))
    }

    fn local_mut(&mut self, index: u8) -> JResult<&mut JValue> {
        self.locals
            .get_mut(index as usize)
            .ok_or_else(|| FnError("local does not exist".to_string()))
    }
}

pub struct JClass {
    constant_pool: ConstantPool,
}

pub struct FnCall {
    cls: Arc<LoadedClass>,
    f: Arc<LoadedMethod>,
    pc: u16,
    sp: usize,
    lp: usize,
}

pub enum FnAction {
    Return(JValue),
    ReturnVoid,
    Yield,
    Breakpoint,
    Call(u16),
}

pub struct FnError(String);

pub type JResult<T> = Result<T, FnError>;
pub type FnResult = Result<FnAction, FnError>;

impl FnCall {
    pub fn new(cls: Arc<LoadedClass>, f: Arc<LoadedMethod>, engine: &mut JEngine) -> Self {
        let lp = engine.locals.len();
        engine.locals.reserve(f.max_locals as usize);
        for _i in 0..f.args {
            engine
                .locals
                .push(engine.stack.pop().expect("stack underflow"));
        }
        for _i in f.args..f.max_locals {
            engine.locals.push(JValue::Invalid); // TODO: use right types
        }

        let sp = engine.stack.len();
        engine.stack.reserve(f.max_stack as usize);

        Self {
            cls,
            f,
            pc: 0,
            sp,
            lp,
        }
    }

    pub fn invoke(&mut self, engine: &mut JEngine) -> FnResult {
        let code = &self.f.code;

        loop {
            match code.get(self.pc as usize).expect("invalid p... counter") {
                Opcode::Aaload => {
                    let (array_ref, index) = engine.pop2()?;
                    let index = match index {
                        JValue::Int(i) => i as i32,
                        JValue::Short(i) => i as i32,
                        JValue::Byte(i) => i as i32,
                        _ => panic!("wrong index type"),
                    };

                    match array_ref {
                        JValue::ObjectArray(arr) => {
                            if let Some(value) = arr.borrow().get(index as usize) {
                                engine.stack.push(JValue::Reference(value.clone()));
                            } else {
                                return Err(FnError("ArrayIndexOutOfBoundsException".into()));
                            }
                        }
                        _ => panic!("wrong array type"),
                    }
                }
                Opcode::Aastore => {
                    let (array_ref, index, value) = engine.pop3()?;
                    let index = match index {
                        JValue::Int(i) => i as i32,
                        JValue::Short(i) => i as i32,
                        JValue::Byte(i) => i as i32,
                        _ => return Err(FnError("wrong index type".into())),
                    };
                    let value = value.as_reference()?;

                    match array_ref {
                        JValue::ObjectArray(arr) => {
                            if let Some(mut value_ref) = arr.borrow_mut().get_mut(index as usize) {
                                *value_ref = value.clone();
                            } else {
                                return Err(FnError("ArrayIndexOutOfBoundsException".into()));
                            }
                        }
                        _ => return Err(FnError("wrong array type".into())),
                    }
                }
                Opcode::Aload(var) => match engine.locals.get(self.lp + *var as usize) {
                    Some(JValue::Reference(r)) => engine.stack.push(JValue::Reference(r.clone())),
                    Some(_) => return Err(FnError("Wrong type".into())),
                    None => return Err(FnError("Non existing var".into())),
                },

                Opcode::Pop => {
                    let _ = engine.pop()?;
                }
                Opcode::Pop2 => {
                    let _ = engine.pop2()?;
                }
                Opcode::Dup => engine.push(engine.top()?.clone())?,

                Opcode::IconstM1 => engine.push(JValue::Int(-1))?,
                Opcode::Iconst0 => engine.push(JValue::Int(0))?,
                Opcode::Iconst1 => engine.push(JValue::Int(1))?,
                Opcode::Iconst2 => engine.push(JValue::Int(2))?,
                Opcode::Iconst3 => engine.push(JValue::Int(3))?,
                Opcode::Iconst4 => engine.push(JValue::Int(4))?,
                Opcode::Iconst5 => engine.push(JValue::Int(5))?,
                Opcode::Lconst0 => engine.push(JValue::Long(0))?,
                Opcode::Lconst1 => engine.push(JValue::Long(1))?,
                Opcode::Dconst0 => engine.push(JValue::Double(0.0))?,
                Opcode::Dconst1 => engine.push(JValue::Double(1.0))?,
                Opcode::Fconst0 => engine.push(JValue::Float(0.0))?,
                Opcode::Fconst1 => engine.push(JValue::Float(1.0))?,
                Opcode::Fconst2 => engine.push(JValue::Float(2.0))?,
                Opcode::AconstNull => engine.push(JValue::Reference(None))?,
                Opcode::Ldc(index) => {
                    unimplemented!()
                    // engine.push(
                    //     self.cls
                    //         .constant_pool
                    //         .get(ConstantIndex(*index as u16))
                    //         .ok_or_else(|| FnError("non-existing constant".into()))?
                    //         .into(),
                    // )?;
                }
                Opcode::LdcW(index) => {
                    unimplemented!()
                    // engine.push(self.f.class.constant_pool.get(ConstantIndex(*index)).into())?;
                }

                Opcode::F2i => {
                    let j_value = JValue::Int(engine.pop()?.as_float()? as i32);
                    engine.push(j_value)?
                }
                Opcode::F2l => {
                    let value = JValue::Long(engine.pop()?.as_float()? as i64);
                    engine.push(value)?
                }
                Opcode::F2d => {
                    let value = JValue::Double(engine.pop()?.as_float()? as f64);
                    engine.push(value)?
                }

                Opcode::D2i => {
                    let value = JValue::Int(engine.pop()?.as_double()? as i32);
                    engine.push(value)?
                }
                Opcode::D2l => {
                    let value = JValue::Long(engine.pop()?.as_double()? as i64);
                    engine.push(value)?
                }
                Opcode::D2f => {
                    let value = JValue::Float(engine.pop()?.as_double()? as f32);
                    engine.push(value)?
                }

                Opcode::L2i => {
                    let value = JValue::Int(engine.pop()?.as_long()? as i32);
                    engine.push(value)?
                }
                Opcode::L2f => {
                    let value = JValue::Float(engine.pop()?.as_long()? as f32);
                    engine.push(value)?
                }
                Opcode::L2d => {
                    let value = JValue::Double(engine.pop()?.as_long()? as f64);
                    engine.push(value)?
                }

                Opcode::I2l => {
                    let value = JValue::Long(engine.pop()?.as_int()? as i64);
                    engine.push(value)?
                }
                Opcode::I2f => {
                    let value = JValue::Float(engine.pop()?.as_int()? as f32);
                    engine.push(value)?
                }
                Opcode::I2d => {
                    let value = JValue::Double(engine.pop()?.as_int()? as f64);
                    engine.push(value)?
                }

                Opcode::Fadd => {
                    let (lhs, rhs) = engine.pop2()?;
                    engine.push(JValue::Float(lhs.as_float()? + rhs.as_float()?))?;
                }
                Opcode::Fsub => {
                    let (lhs, rhs) = engine.pop2()?;
                    engine.push(JValue::Float(lhs.as_float()? - rhs.as_float()?))?;
                }

                Opcode::Ret(index) => {
                    self.pc = engine.local(*index)?.as_int()? as u16;
                    continue;
                }

                Opcode::Areturn => {
                    return match engine.pop()? {
                        value @ JValue::Reference(_) => Ok(FnAction::Return(value)),
                        _ => Err(FnError("expected reference type at stack top".into())),
                    };
                }
                Opcode::Return => return Ok(FnAction::ReturnVoid),

                Opcode::Nop => (),
                Opcode::Breakpoint => return Ok(FnAction::Breakpoint),

                _ => return Err(FnError("unsupported opcode".into())),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
