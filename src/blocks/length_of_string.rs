use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct LengthOfString {
    id: Id,
    value: Value,
}
impl Block for LengthOfString {
    fn run(&self, pointer: usize, memory: &mut Memory, _ctx: &mut Context) -> BlockReturn {
        let length = self
            .value
            .take_raw_value(memory)
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
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (value, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            LengthOfString {
                id: block.id.clone().into(),
                value,
            }
            .into(),
        );
        blocks
    }
}
