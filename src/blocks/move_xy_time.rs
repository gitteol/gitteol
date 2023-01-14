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
        let time = memory
            .cache(&self.id, "time", &self.time)
            .as_number()
            .unwrap();
        let x = memory.cache(&self.id, "x", &self.x).as_number().unwrap();
        let y = memory.cache(&self.id, "y", &self.y).as_number().unwrap();

        let mut this = ctx.objects.get_mut(*ctx.owner).unwrap();

        let delta = memory
            .entry(&self.id, "delta")
            .or_insert(Value::Number(0.0))
            .as_number_mut()
            .unwrap();

        let this_delta = (ctx.time.delta_seconds() / time).min(1.0);

        let new_delta = *delta + this_delta;

        if new_delta < 1.0 {
            this.translation.x += this_delta * x;
            this.translation.y += this_delta * y;
            *delta = new_delta;
            BlockReturn {
                pointer,
                is_continue: true,
                return_value: None,
            }
        } else {
            let this_delta = 1.0 - *delta;
            this.translation.x += this_delta * x;
            this.translation.y += this_delta * y;

            memory.remove_many(&self.id, &["delta", "time", "x", "y"]);
            BlockReturn::basic(pointer)
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
