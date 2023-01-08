use crate::common::Id;

use super::{parse_param, Block, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct LocateXY {
    id: Id,
    x: Value,
    y: Value,
}

impl LocateXY {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (x, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let (y, mut param_blocks) = parse_param(&block.params[1]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            LocateXY {
                id: block.id.clone().into(),
                x,
                y,
            }
            .into(),
        );

        blocks
    }
}

impl Block for LocateXY {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let x = self.x.to_raw_value(memory).unwrap().as_number().unwrap();
        let y = self.y.to_raw_value(memory).unwrap().as_number().unwrap();

        ctx.object.translation.x = x;
        ctx.object.translation.y = y;

        super::BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
