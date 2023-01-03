use bevy::prelude::info;

use crate::common::Id;

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct MoveXYTime {
    id: Id,
    time: Value,
    x: Value,
    y: Value,
}

impl MoveXYTime {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (time, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let (x, mut param_blocks) = parse_param(&block.params[1]).unwrap();
        blocks.append(&mut param_blocks);

        let (y, mut param_blocks) = parse_param(&block.params[2]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            MoveXYTime {
                id: block.id.clone().into(),
                time,
                x,
                y,
            }
            .into(),
        );

        blocks
    }
}

impl Block for MoveXYTime {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let time = self.time.to_raw_value(memory).unwrap().as_number().unwrap();
        let x = self.x.to_raw_value(memory).unwrap().as_number().unwrap();
        let y = self.y.to_raw_value(memory).unwrap().as_number().unwrap();

        let delta = memory
            .entry(&self.id, "delta")
            .or_insert(Value::Number(0.0))
            .as_number_mut()
            .unwrap();

        let this_delta = (ctx.time.delta_seconds() / time).min(1.0);

        let new_delta = *delta + this_delta;

        if new_delta < 1.0 {
            ctx.object.translation.x += this_delta * x;
            ctx.object.translation.y += this_delta * y;
            *delta = new_delta;
            BlockReturn {
                pointer,
                is_continue: true,
                return_value: None,
            }
        } else {
            let this_delta = 1.0 - *delta;
            ctx.object.translation.x += this_delta * x;
            ctx.object.translation.y += this_delta * y;

            info!("{:#?}", ctx.object.translation);

            memory.remove(&self.id, "delta");
            BlockReturn::basic(pointer)
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
