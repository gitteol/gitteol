use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct WaitSecond {
    id: Id,
    second: Value,
}
impl Block for WaitSecond {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn {
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

        *delta += res.time.delta_seconds();

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
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let second = block.params[0].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(WaitSecond {
            id: Id::from_str(&block.id),
            second,
        }));
        blocks
    }
}
