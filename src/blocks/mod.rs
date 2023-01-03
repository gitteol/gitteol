mod _if;
mod boolean_basic_operator;
mod calc_basic;
mod change_variable;
mod get_variable;
mod length_of_string;
mod move_direction;
mod move_x;
mod move_xy_time;
mod move_y;
mod repeat_basic;
mod repeat_inf;
mod set_variable;
mod wait_second;

use std::str::FromStr;

use bevy::prelude::*;
use dotent::project::script::Param;
use enum_dispatch::enum_dispatch;
use strum::{EnumDiscriminants, EnumString};

use crate::{
    code::{Context, Memory},
    common::Id,
};

use self::{
    _if::If,
    boolean_basic_operator::BooleanBasicOperator,
    calc_basic::CalcBasic,
    change_variable::ChangeVariable,
    get_variable::GetVariable,
    length_of_string::LengthOfString,
    move_direction::MoveDirection,
    move_x::MoveX,
    move_xy_time::MoveXYTime,
    move_y::MoveY,
    repeat_basic::{RepeatBasic, RepeatBasicEnd},
    repeat_inf::RepeatInf,
    set_variable::SetVariable,
    wait_second::WaitSecond,
};

#[enum_dispatch]
#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumString))]
#[strum_discriminants(strum(serialize_all = "snake_case"))]
#[strum_discriminants(name(BlockType))]
#[strum_discriminants(vis(pub(crate)))]
pub(crate) enum BlockEnum {
    MoveDirection,
    WaitSecond,
    RepeatBasic,
    RepeatBasicEnd,
    LengthOfString,
    SetVariable,
    GetVariable,
    ChangeVariable,
    #[strum_discriminants(strum(serialize = "_if"))]
    If,
    BooleanBasicOperator,
    MoveX,
    MoveY,
    RepeatInf,
    CalcBasic,
    MoveXYTime,
}
impl BlockType {
    pub(crate) fn build(&self, block: &dotent::project::script::Block) -> BlockVec {
        match self {
            BlockType::MoveDirection => MoveDirection::build(block),
            BlockType::WaitSecond => WaitSecond::build(block),
            BlockType::RepeatBasic => RepeatBasic::build(block),
            BlockType::RepeatBasicEnd => unreachable!(),
            BlockType::RepeatInf => RepeatInf::build(block),
            BlockType::LengthOfString => LengthOfString::build(block),
            BlockType::SetVariable => SetVariable::build(block),
            BlockType::GetVariable => GetVariable::build(block),
            BlockType::ChangeVariable => ChangeVariable::build(block),
            BlockType::If => If::build(block),
            BlockType::BooleanBasicOperator => BooleanBasicOperator::build(block),
            BlockType::MoveX => MoveX::build(block),
            BlockType::MoveY => MoveY::build(block),
            BlockType::CalcBasic => CalcBasic::build(block),
            BlockType::MoveXYTime => MoveXYTime::build(block),
        }
    }
}

#[derive(Debug, EnumString)]
pub(crate) enum LiteralBlockType {
    #[strum(serialize = "number")]
    Number,
    #[strum(serialize = "text")]
    Text,
    True,
}

#[derive(Clone, Debug)]
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
            Self::String(val) => {
                let new = val.parse::<f32>().or(Err("cannot convert as number"))?;
                *self = Value::Number(new);
                self.as_number_mut()
            }
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
impl From<dotent::project::variable::Value> for Value {
    fn from(value: dotent::project::variable::Value) -> Self {
        match value {
            dotent::project::variable::Value::Number(val) => Value::Number(val),
            dotent::project::variable::Value::String(val) => Value::String(val),
        }
    }
}

pub(crate) fn parse_block(raw_block: &dotent::project::script::Block) -> BlockVec {
    if let Ok(block_type) = BlockType::from_str(&raw_block.block_type) {
        block_type.build(raw_block)
    } else {
        warn!("unsupported block: {}", raw_block.block_type);
        Vec::new()
    }
}
fn parse_param(param: &Param) -> Option<(Value, BlockVec)> {
    let mut blocks = Vec::new();
    let val = match param {
        Param::Block(block) => {
            if let Ok(block_type) = BlockType::from_str(&block.block_type) {
                let mut param_blocks = block_type.build(block);
                let last_id = param_blocks.last().unwrap().get_id().clone();
                blocks.append(&mut param_blocks);
                Value::Memory((last_id, "return_value".to_string()))
            } else if LiteralBlockType::from_str(&block.block_type).is_ok() {
                return parse_param(&block.params[0]);
            } else {
                warn!("unsupported block: {}", block.block_type);
                return None;
            }
        }
        Param::Number(val) => Value::Number(*val),
        Param::String(val) => Value::String(val.to_string()),
        Param::Bool(val) => Value::Bool(*val),
        Param::Null => return None,
    };
    Some((val, blocks))
}
fn parse_statements(script: &dotent::project::script::Script) -> Vec<BlockVec> {
    let mut codes = Vec::new();
    for code in script.0.iter() {
        let mut blocks = Vec::new();
        for block in code {
            blocks.append(&mut parse_block(block));
        }
        codes.push(blocks);
    }
    codes
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
pub(crate) type BlockVec = Vec<BlockEnum>;

#[enum_dispatch(BlockEnum)]
pub(crate) trait Block {
    fn run(&self, pointer: usize, memory: &mut Memory, ctx: &mut Context) -> BlockReturn;
    fn get_id(&self) -> &Id;
}
