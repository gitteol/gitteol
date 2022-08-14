use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct If {
    id: Id,
    condition: Value,
    statements_length: usize,
}
impl Block for If {
    fn run(&self, pointer: usize, memory: &mut Memory, _res: &mut Resources) -> BlockReturn {
        let condition = self
            .condition
            .to_raw_value(memory)
            .unwrap()
            .as_bool()
            .unwrap();
        if condition {
            BlockReturn {
                pointer: pointer + 1,
                is_continue: false,
                return_value: None,
            }
        } else {
            BlockReturn {
                pointer: pointer + self.statements_length + 1,
                is_continue: false,
                return_value: None,
            }
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl If {
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let condition = block.params[0].to_arg(&mut blocks).unwrap();
        let mut statements = block.statements.to_statements();
        let statement_len = statements[0].len();
        blocks.push(Box::new(If {
            id: Id::from_str(&block.id),
            condition,
            statements_length: statement_len,
        }));
        blocks.append(&mut statements[0]);
        blocks
    }
}
