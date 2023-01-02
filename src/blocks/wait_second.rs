use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct WaitSecond {
    id: Id,
    second: Value,
}
impl Block for WaitSecond {
    fn run(&self, pointer: usize, memory: &mut Memory, ctx: &mut Context) -> BlockReturn {
        let second = self
            .second
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();

        let delta = memory
            .entry(&self.id, "delta")
            .or_insert(Value::Number(0.0))
            .as_number_mut()
            .unwrap();

        *delta += ctx.time.delta_seconds();

        if *delta >= second {
            memory.remove(&self.id, "delta");
            BlockReturn {
                pointer: pointer + 1,
                is_continue: false,
                return_value: None,
            }
        } else {
            BlockReturn {
                pointer,
                is_continue: true,
                return_value: None,
            }
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl WaitSecond {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (second, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            WaitSecond {
                id: block.id.clone().into(),
                second,
            }
            .into(),
        );
        blocks
    }
}
