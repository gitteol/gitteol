use crate::{code::Memory, Id};

pub(crate) mod functions;

#[derive(Clone, Copy, Debug)]
pub(crate) enum BlockType {
    MoveDirection,
    WaitSecond,
    RepeatBasic,
    RepeatBasicEnd,
    LengthOfString,
    SetVariable,
    GetVariable,
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    String(String),
    Number(f32),
    Bool(bool),
    Memory((Id, String)),
}
impl Value {
    fn as_string(&self) -> Result<String, &str> {
        match self {
            Self::String(val) => Ok(val.to_string()),
            Self::Number(val) => Ok(val.to_string()),
            Self::Bool(val) => Ok(val.to_string()),
            _ => Err("cannot convert as string"),
        }
    }
    fn as_number(&self) -> Result<f32, &str> {
        match self {
            Self::Number(val) => Ok(*val),
            Self::String(val) => val.parse::<f32>().or(Err("cannot convert as number")),
            Self::Bool(val) => Ok(if *val { 1.0 } else { 0.0 }),
            _ => Err("cannot convert as number"),
        }
    }
    fn as_number_mut(&mut self) -> Result<&mut f32, &str> {
        match self {
            Self::Number(val) => Ok(val),
            _ => Err("cannot convert as mutable number"),
        }
    }
    fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(*val),
            _ => None,
        }
    }
    fn to_raw_value<'a>(&'a self, memory: &'a Memory) -> Option<&'a Value> {
        let mut value = Some(self);
        while let Some(Value::Memory((id, label))) = value {
            value = memory.get(id, label);
        }
        value
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Block {
    pub(crate) id: Id,
    pub(crate) block_type: BlockType,
    pub(crate) args: Vec<Value>,
    pub(crate) extra: Vec<Value>,
}

pub(crate) struct BlockReturn {
    pub(crate) pointer: usize,
    pub(crate) is_continue: bool,
    pub(crate) return_value: Option<Value>,
}

impl BlockReturn {
    fn basic(pointer: usize) -> BlockReturn {
        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: None,
        }
    }
}
