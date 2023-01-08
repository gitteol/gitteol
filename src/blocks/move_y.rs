use crate::common::Id;

use super::{parse_param, Block, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct MoveY {
    id: Id,
    amount: Value,
}

impl MoveY {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (amount, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            MoveY {
                id: block.id.clone().into(),
                amount,
            }
            .into(),
        );

        blocks
    }
}

impl Block for MoveY {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let amount = self
            .amount
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();

        let mut this = ctx.objects.get_mut(*ctx.owner).unwrap();

        this.translation.y += amount;

        super::BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
