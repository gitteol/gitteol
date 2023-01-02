use crate::{
    code::{Memory, Resources},
    common::Id,
};

use super::{parse_param, parse_statements, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct RepeatBasic {
    id: Id,
    iter_num: Value,
    statements_length: usize,
}
impl Block for RepeatBasic {
    fn run(&self, pointer: usize, memory: &mut Memory, _res: &mut Resources) -> BlockReturn {
        let iter_num = self
            .iter_num
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();
        let count = memory
            .entry(&self.id, "count")
            .or_insert(Value::Number(0.0))
            .as_number_mut()
            .unwrap();
        if *count < iter_num {
            *count += 1.0;
            BlockReturn {
                pointer: pointer + 1,
                is_continue: false,
                return_value: None,
            }
        } else {
            memory.remove(&self.id, "count");
            BlockReturn {
                pointer: pointer + self.statements_length + 2,
                is_continue: false,
                return_value: None,
            }
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl RepeatBasic {
    pub(crate) fn new(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (iter_num, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let mut statements = parse_statements(&block.statements);
        let statement_len = statements[0].len();

        blocks.push(
            RepeatBasic {
                id: block.id.clone().into(),
                iter_num,
                statements_length: statement_len,
            }
            .into(),
        );
        blocks.append(&mut statements[0]);
        blocks.push(
            RepeatBasicEnd {
                id: block.id.clone().into(),
                statements_length: statement_len,
            }
            .into(),
        );
        blocks
    }
}

#[derive(Clone)]
pub(crate) struct RepeatBasicEnd {
    id: Id,
    statements_length: usize,
}
impl Block for RepeatBasicEnd {
    fn run(&self, pointer: usize, _memory: &mut Memory, _res: &mut Resources) -> BlockReturn {
        BlockReturn {
            pointer: pointer - self.statements_length - 1,
            is_continue: true,
            return_value: None,
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
