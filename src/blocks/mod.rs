use crate::{code::Memory, Id};

pub(crate) mod functions;

#[derive(Clone, Debug)]
pub(crate) enum BlockType {
    MoveDirection,
    WaitSecond,
    RepeatBasic,
    RepeatBasicEnd,
    LengthOfString,
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    String(String),
    Number(f32),
    Boolean(bool),
    Memory((Id, String)),
}
impl Value {
    fn string(&self) -> Option<&str> {
        match self {
            Self::String(val) => Some(&val[..]),
            _ => None,
        }
    }
    fn number(&self) -> Option<f32> {
        match self {
            Self::Number(val) => Some(*val),
            _ => None,
        }
    }
    fn number_mut(&mut self) -> Option<&mut f32> {
        match self {
            Self::Number(val) => Some(val),
            _ => None,
        }
    }
    fn boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(val) => Some(*val),
            _ => None,
        }
    }
    fn memory<'a>(&self, memory: &'a Memory) -> Option<&'a Value> {
        match self {
            Self::Memory((id, label)) => memory.get(id, label),
            _ => None,
        }
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
