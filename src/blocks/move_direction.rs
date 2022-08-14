use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct MoveDirection {
    id: Id,
    amount: Value,
}
impl Block for MoveDirection {
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
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let amount = block.params[0].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(MoveDirection {
            id: Id::from_str(&block.id),
            amount,
        }));
        blocks
    }
}
