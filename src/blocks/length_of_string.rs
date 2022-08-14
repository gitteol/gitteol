use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct LengthOfString {
    id: Id,
    value: Value,
}
impl Block for LengthOfString {
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
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let value = block.params[0].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(LengthOfString {
            id: Id::from_str(&block.id),
            value,
        }));
        blocks
    }
}
