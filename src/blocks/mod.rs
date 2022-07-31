use dyn_clone::DynClone;
use serde::Deserialize;

use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum BlockType {
    MoveDirection,
    WaitSecond,
    RepeatBasic,
    RepeatBasicEnd,
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
            BlockType::RepeatBasicEnd => RepeatBasicEnd::new,
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

// #[derive(Clone, Debug)]
// pub(crate) struct Block {
//     pub(crate) id: Id,
//     pub(crate) block_type: BlockType,
//     pub(crate) args: Vec<Value>,
//     pub(crate) extra: Vec<Value>,
// }

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
pub(crate) type BlockVec = Vec<Box<dyn BlockDef + Send + Sync>>;

pub(crate) trait BlockDef: DynClone {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn;
    fn get_id(&self) -> &Id;
}
// TODO: change to use struct +BlockDef instead of Block struct at parsing state.
// TODO: rename BlockDef to Block

dyn_clone::clone_trait_object!(BlockDef);

#[derive(Clone)]
struct MoveDirection {
    id: Id,
    amount: Value,
}
impl BlockDef for MoveDirection {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let amount = self
            .amount
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();
        res.object.transform.translation.x += amount;

        BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}

impl MoveDirection {
    fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let amount = block.params[0].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(MoveDirection {
            id: Id::from_str(&block.id),
            amount,
        }));
        blocks
    }
}

#[derive(Clone)]
struct WaitSecond {
    id: Id,
    second: Value,
}
impl BlockDef for WaitSecond {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let second = self
            .second
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();

        let delta = memory
            .entry(&self.id, "delta")
            .or_insert(Value::Number(0.0))
            .as_number_mut()
            .unwrap();

        *delta += res.time.delta_seconds();

        if *delta >= second {
            memory.remove(&self.id, "delta");
            BlockReturn {
                pointer: pointer + 1,
                is_continue: false,
                return_value: None,
            }
        } else {
            BlockReturn {
                pointer,
                is_continue: true,
                return_value: None,
            }
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl WaitSecond {
    fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let second = block.params[0].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(WaitSecond {
            id: Id::from_str(&block.id),
            second,
        }));
        blocks
    }
}

#[derive(Clone)]
struct RepeatBasic {
    id: Id,
    iter_num: Value,
    statements_length: usize,
}
impl BlockDef for RepeatBasic {
    fn run(&self, pointer: usize, memory: &mut Memory, _res: &mut Resources) -> BlockReturn {
        let iter_num = self
            .iter_num
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();
        let count = memory
            .entry(&self.id, "count")
            .or_insert(Value::Number(0.0))
            .as_number_mut()
            .unwrap();
        if *count < iter_num {
            *count += 1.0;
            BlockReturn {
                pointer: pointer + 1,
                is_continue: false,
                return_value: None,
            }
        } else {
            memory.remove(&self.id, "count");
            BlockReturn {
                pointer: pointer + self.statements_length + 2,
                is_continue: false,
                return_value: None,
            }
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}

impl RepeatBasic {
    fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let iter_num = block.params[0].to_arg(&mut blocks).unwrap();
        let mut statements = block.statements.to_statements();
        let statement_len = statements[0].len();
        blocks.push(Box::new(RepeatBasic {
            id: Id::from_str(&block.id),
            iter_num,
            statements_length: statement_len,
        }));
        blocks.append(&mut statements[0]);
        blocks.push(Box::new(RepeatBasicEnd {
            id: Id::from_str(&block.id),
            statements_length: statement_len,
        }));
        blocks
    }
}

#[derive(Clone)]
struct RepeatBasicEnd {
    id: Id,
    statements_length: usize,
}
impl BlockDef for RepeatBasicEnd {
    fn run(&self, pointer: usize, _memory: &mut Memory, _res: &mut Resources) -> BlockReturn {
        BlockReturn {
            pointer: pointer - self.statements_length - 1,
            is_continue: true,
            return_value: None,
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl RepeatBasicEnd {
    fn new(_block: &RawBlock) -> BlockVec {
        Vec::new()
    }
}

#[derive(Clone)]
struct LengthOfString {
    id: Id,
    value: Value,
}
impl BlockDef for LengthOfString {
    fn run(&self, pointer: usize, memory: &mut Memory, _res: &mut Resources) -> BlockReturn {
        let length = self
            .value
            .to_raw_value(memory)
            .unwrap()
            .as_string()
            .unwrap()
            .chars()
            .count();

        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: Some(Value::Number(length as f32)),
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl LengthOfString {
    fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let value = block.params[0].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(LengthOfString {
            id: Id::from_str(&block.id),
            value,
        }));
        blocks
    }
}

#[derive(Clone)]
struct SetVariable {
    id: Id,
    variable_id: String,
    value: Value,
}
impl BlockDef for SetVariable {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let variable_entity = res.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let value = self.value.to_raw_value(memory).unwrap();

        let mut variable = res.variables.get_mut(*variable_entity).unwrap();

        variable.value = value.clone();

        BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl SetVariable {
    fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let variable_id = block.params[0]
            .to_arg(&mut blocks)
            .unwrap()
            .as_string()
            .unwrap();
        let value = block.params[1].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(SetVariable {
            id: Id::from_str(&block.id),
            variable_id,
            value,
        }));
        blocks
    }
}

#[derive(Clone)]
struct GetVariable {
    id: Id,
    variable_id: String,
}
impl BlockDef for GetVariable {
    fn run(&self, pointer: usize, _memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let variable_entity = res.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let variable = res.variables.get(*variable_entity).unwrap();

        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: Some(variable.value.clone()),
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl GetVariable {
    fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let variable_id = block.params[0]
            .to_arg(&mut blocks)
            .unwrap()
            .as_string()
            .unwrap();
        blocks.push(Box::new(GetVariable {
            id: Id::from_str(&block.id),
            variable_id,
        }));
        blocks
    }
}
