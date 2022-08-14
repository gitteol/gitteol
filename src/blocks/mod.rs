mod get_variable;
mod length_of_string;
mod move_direction;
mod repeat_basic;
mod set_variable;
mod wait_second;

use dyn_clone::DynClone;
use serde::Deserialize;

use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use self::{
    get_variable::GetVariable, length_of_string::LengthOfString, move_direction::MoveDirection,
    repeat_basic::RepeatBasic, set_variable::SetVariable, wait_second::WaitSecond,
};

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BlockType {
    MoveDirection,
    WaitSecond,
    RepeatBasic,
    LengthOfString,
    SetVariable,
    GetVariable,
}
impl BlockType {
    pub(crate) fn new_block(&self) -> fn(&RawBlock) -> BlockVec {
        match self {
            BlockType::MoveDirection => MoveDirection::new,
            BlockType::WaitSecond => WaitSecond::new,
            BlockType::RepeatBasic => RepeatBasic::new,
            BlockType::LengthOfString => LengthOfString::new,
            BlockType::SetVariable => SetVariable::new,
            BlockType::GetVariable => GetVariable::new,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) enum LiteralBlockType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "text")]
    Text,
    True,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Value {
    String(String),
    Number(f32),
    Bool(bool),
    Memory((Id, String)),
}
impl Value {
    pub(crate) fn as_string(&self) -> Result<String, &str> {
        match self {
            Self::String(val) => Ok(val.to_string()),
            Self::Number(val) => Ok(val.to_string()),
            Self::Bool(val) => Ok(val.to_string()),
            _ => Err("cannot convert as string"),
        }
    }
    pub(crate) fn as_number(&self) -> Result<f32, &str> {
        match self {
            Self::Number(val) => Ok(*val),
            Self::String(val) => val.parse::<f32>().or(Err("cannot convert as number")),
            Self::Bool(val) => Ok(if *val { 1.0 } else { 0.0 }),
            _ => Err("cannot convert as number"),
        }
    }
    pub(crate) fn as_number_mut(&mut self) -> Result<&mut f32, &str> {
        match self {
            Self::Number(val) => Ok(val),
            _ => Err("cannot convert as mutable number"),
        }
    }
    pub(crate) fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(*val),
            _ => None,
        }
    }
    pub(crate) fn to_raw_value<'a>(&'a self, memory: &'a Memory) -> Option<&'a Value> {
        let mut value = Some(self);
        while let Some(Value::Memory((id, label))) = value {
            value = memory.get(id, label);
        }
        value
    }
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
pub(crate) type BlockVec = Vec<Box<dyn Block + Send + Sync>>;

pub(crate) trait Block: DynClone {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn;
    fn get_id(&self) -> &Id;
}

dyn_clone::clone_trait_object!(Block);
