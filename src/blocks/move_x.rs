use crate::common::Id;

use super::{parse_param, Block, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct MoveX {
    id: Id,
    amount: Value,
}

impl MoveX {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (amount, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            MoveX {
                id: block.id.clone().into(),
                amount,
            }
            .into(),
        );

        blocks
    }
}

impl Block for MoveX {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let amount = self
            .amount
            .take_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();

        let mut this = ctx.objects.get_mut(*ctx.owner).unwrap();
        this.translation.x += amount;

        super::BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
