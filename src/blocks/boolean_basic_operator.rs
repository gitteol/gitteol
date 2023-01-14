use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct BooleanBasicOperator {
    id: Id,
    left: Value,
    op: String,
    right: Value,
}
impl Block for BooleanBasicOperator {
    fn run(&self, pointer: usize, memory: &mut Memory, _ctx: &mut Context) -> BlockReturn {
        let left = self.left.take_raw_value(memory).unwrap();
        let right = self.right.take_raw_value(memory).unwrap();

        let result = match (left.as_number(), right.as_number()) {
            (Ok(l), Ok(r)) => match &self.op[..] {
                "EQUAL" => l == r,
                "NOT_EQUAL" => l != r,
                "GREATER" => l > r,
                "LESS" => l < r,
                "GREATER_OR_EQUAL" => l >= r,
                "LESS_OR_EQUAL" => l <= r,
                _ => unreachable!(),
            },
            _ => match &self.op[..] {
                "EQUAL" => left.as_string().unwrap() == right.as_string().unwrap(),
                "NOT_EQUAL" => left.as_string().unwrap() != right.as_string().unwrap(),
                _ => false,
            },
        };

        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: Some(Value::Bool(result)),
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl BooleanBasicOperator {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (left, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let (op, _) = parse_param(&block.params[1]).unwrap();
        let op = op.as_string().unwrap();

        let (right, mut param_blocks) = parse_param(&block.params[2]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            BooleanBasicOperator {
                id: block.id.clone().into(),
                left,
                op,
                right,
            }
            .into(),
        );
        blocks
    }
}
