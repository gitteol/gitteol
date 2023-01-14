use crate::{
    code::{Context, Memory},
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
    fn run(&self, pointer: usize, memory: &mut Memory, _ctx: &mut Context) -> BlockReturn {
        let iter_num = memory
            .cache(&self.id, "iter_num", &self.iter_num)
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
            memory.remove_many(&self.id, &["count", "iter_num"]);
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
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
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
    pub(crate) id: Id,
    pub(crate) statements_length: usize,
}
impl Block for RepeatBasicEnd {
    fn run(&self, pointer: usize, _memory: &mut Memory, _res: &mut Context) -> BlockReturn {
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
