use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct MoveDirection {
    id: Id,
    amount: Value,
}
impl Block for MoveDirection {
    fn run(&self, pointer: usize, memory: &mut Memory, ctx: &mut Context) -> BlockReturn {
        let amount = self
            .amount
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();
        ctx.object.translation.x += amount;

        BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl MoveDirection {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let (amount, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);
        blocks.push(
            MoveDirection {
                id: block.id.clone().into(),
                amount,
            }
            .into(),
        );
        blocks
    }
}
