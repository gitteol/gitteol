use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct CalcBasic {
    id: Id,
    left: Value,
    op: String,
    right: Value,
}
impl Block for CalcBasic {
    fn run(&self, pointer: usize, memory: &mut Memory, _ctx: &mut Context) -> BlockReturn {
        let left = self.left.take_raw_value(memory).unwrap();
        let right = self.right.take_raw_value(memory).unwrap();

        let left_as_num = left.as_number();
        let right_as_num = right.as_number();

        let result = if (left_as_num.is_err() || right_as_num.is_err()) && self.op == "PLUS" {
            Value::String(format!(
                "{}{}",
                left.as_string().unwrap(),
                right.as_string().unwrap()
            ))
        } else {
            let l = left_as_num.unwrap_or(0.0);
            let r = right_as_num.unwrap_or(0.0);

            Value::Number(match &self.op[..] {
                "PLUS" => l + r,
                "MINUS" => l - r,
                "MULTI" => l * r,
                "DIVIDE" => l / r,
                _ => unreachable!(),
            })
        };

        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: Some(result),
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl CalcBasic {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (left, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let (op, _) = parse_param(&block.params[1]).unwrap();
        let op = op.as_string().unwrap();

        let (right, mut param_blocks) = parse_param(&block.params[2]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            CalcBasic {
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
