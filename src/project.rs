use bevy::reflect::TypeUuid;
use serde::Deserialize;

use crate::{
    blocks::{BlockType, BlockVec, LiteralBlockType, Value},
    common::{Id, LocalPos},
    event::EventType,
    variable::{Variable, VariableType},
};

#[derive(Deserialize, Debug, TypeUuid)]
#[uuid = "be3c1877-7c8e-4872-bd28-a6de720c69b9"]
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

impl RawScript {
    pub(crate) fn parse(script: &str) -> serde_json::Result<RawScript> {
        serde_json::from_str(script)
    }
    pub(crate) fn to_statements(&self) -> Vec<BlockVec> {
        let mut codes = Vec::new();
        for code in self.0.iter() {
            let mut blocks = Vec::new();
            for block in code {
                blocks.append(&mut block.to_blocks());
            }
            codes.push(blocks);
        }
        codes
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct RawBlock {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) block_type: RawBlockType,
    pub(crate) params: Vec<RawParam>,
    pub(crate) statements: RawScript,
}

impl RawBlock {
    pub(crate) fn to_blocks(&self) -> BlockVec {
        let mut blocks = Vec::new();
        match self.block_type {
            RawBlockType::Normal(block_type) => {
                let mut block = block_type.new_block()(self);
                blocks.append(&mut block);
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
    pub(crate) fn to_arg(&self, blocks: &mut BlockVec) -> Option<Value> {
        match self {
            RawParam::Block(block) => match block.block_type {
                RawBlockType::Normal(_) => {
                    let mut param_blocks = block.to_blocks();
                    let last_id = param_blocks.last().unwrap().get_id().clone();
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
