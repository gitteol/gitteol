use crate::{
    code::{Memory, Resources},
    common::Id,
};

use super::{parse_param, parse_statements, Block, BlockReturn, BlockVec, Value};

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
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let (condition, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);
        let mut statements = parse_statements(&block.statements);
        let statement_len = statements[0].len();
        blocks.push(
            If {
                id: block.id.clone().into(),
                condition,
                statements_length: statement_len,
            }
            .into(),
        );
        blocks.append(&mut statements[0]);
        blocks
    }
}
