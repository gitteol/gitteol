use serde::Deserialize;

use crate::{
    blocks::{Block, BlockType, LiteralBlockType, Value},
    event::EventType,
    variable::{Variable, VariableType},
    Id, LocalPos,
};

#[derive(Deserialize, Debug)]
pub(crate) struct RawProject {
    pub(crate) objects: Vec<RawObject>,
    pub(crate) variables: Vec<RawVariable>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawObject {
    pub(crate) id: String,
    pub(crate) script: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawScript(pub(crate) Vec<Vec<RawBlock>>);

#[derive(Deserialize, Debug)]
pub(crate) struct RawBlock {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) block_type: RawBlockType,
    pub(crate) params: Vec<RawParam>,
    pub(crate) statements: RawScript,
}

impl RawBlock {
    pub(crate) fn to_blocks(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        match self.block_type {
            RawBlockType::Normal(block_type) => {
                let args = self
                    .params
                    .iter()
                    .filter_map(|param| param.to_arg(&mut blocks))
                    .collect();
                let mut this_block = Block {
                    id: Id::from_str(&self.id),
                    block_type,
                    args,
                    extra: vec![],
                };
                match block_type {
                    BlockType::RepeatBasic => {
                        let mut statements = Vec::new();
                        self.statements.0[0]
                            .iter()
                            .for_each(|block| statements.append(&mut block.to_blocks()));

                        let count = Value::Number(statements.len() as f32);
                        this_block.extra.push(count.clone());

                        blocks.push(this_block);
                        blocks.append(&mut statements);
                        blocks.push(Block {
                            id: Id::from_str(&self.id),
                            block_type: BlockType::RepeatBasicEnd,
                            args: vec![],
                            extra: vec![count],
                        });
                    }
                    _ => blocks.push(this_block),
                };
            }
            _ => unreachable!(),
        }

        blocks
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum RawBlockType {
    Normal(BlockType),
    Event(EventType),
    Literal(LiteralBlockType),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum RawParam {
    Block(RawBlock),
    Number(f32),
    String(String),
    Bool(bool),
    Null,
}

impl RawParam {
    fn to_arg(&self, blocks: &mut Vec<Block>) -> Option<Value> {
        match self {
            RawParam::Block(block) => match block.block_type {
                RawBlockType::Normal(_) => {
                    let mut param_blocks = block.to_blocks();
                    let last_id = param_blocks.last().unwrap().id.clone();
                    blocks.append(&mut param_blocks);
                    Some(Value::Memory((last_id, "return_value".to_string())))
                }
                RawBlockType::Literal(_) => block.params[0].to_arg(blocks),
                _ => unreachable!(),
            },
            RawParam::Number(val) => Some(Value::Number(*val)),
            RawParam::String(val) => Some(Value::String(val.to_string())),
            RawParam::Bool(val) => Some(Value::Bool(*val)),
            RawParam::Null => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawVariable {
    name: String,
    id: String,
    visible: bool,
    value: Value,
    #[serde(rename = "variableType")]
    variable_type: VariableType,
    object: Option<String>,
    x: f32,
    y: f32,
}

impl RawVariable {
    pub(crate) fn to_variable(&self) -> Variable {
        Variable {
            id: Id::from_str(&self.id),
            variable_type: self.variable_type,
            name: self.name.clone(),
            value: self.value.clone(),
            visible: self.visible,
            pos: LocalPos::new(self.x, self.y),
        }
    }
}
