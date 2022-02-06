use crate::{code::Memory, Id};

pub(crate) mod functions;

#[derive(Clone, Debug)]
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
    Boolean(bool),
    Memory((Id, String)),
}
impl Value {
    fn string(&self) -> Result<String, &str> {
        match self {
            Self::String(val) => Ok(val.to_string()),
            Self::Number(val) => Ok(val.to_string()),
            Self::Boolean(val) => Ok(val.to_string()),
            _ => Err("cannot convert as string"),
        }
    }
    fn number(&self) -> Result<f32, &str> {
        match self {
            Self::Number(val) => Ok(*val),
            Self::String(val) => val.parse::<f32>().or(Err("cannot convert as number")),
            Self::Boolean(val) => Ok(if *val { 1.0 } else { 0.0 }),
            _ => Err("cannot convert as number"),
        }
    }
    fn number_mut(&mut self) -> Result<&mut f32, &str> {
        match self {
            Self::Number(val) => Ok(val),
            _ => Err("cannot convert as mutable number"),
        }
    }
    fn boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(val) => Some(*val),
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
}

pub(crate) struct BlockReturn {
    pub(crate) pointer: usize,
    pub(crate) is_continue: bool,
    pub(crate) return_value: Option<Value>,
}
