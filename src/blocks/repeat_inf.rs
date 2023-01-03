use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_statements, repeat_basic::RepeatBasicEnd, Block, BlockReturn, BlockVec};

#[derive(Clone)]
pub(crate) struct RepeatInf {
    id: Id,
}
impl Block for RepeatInf {
    fn run(&self, pointer: usize, _memory: &mut Memory, _ctx: &mut Context) -> BlockReturn {
        BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl RepeatInf {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let mut statements = parse_statements(&block.statements);
        let statement_len = statements[0].len();

        blocks.push(
            RepeatInf {
                id: block.id.clone().into(),
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
