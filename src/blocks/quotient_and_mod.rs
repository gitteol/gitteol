use crate::common::Id;

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct QuotientAndMod {
    id: Id,
    left: Value,
    right: Value,
    operator: Value,
}

impl QuotientAndMod {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (left, mut param_blocks) = parse_param(&block.params[1]).unwrap();
        blocks.append(&mut param_blocks);

        let (right, mut param_blocks) = parse_param(&block.params[3]).unwrap();
        blocks.append(&mut param_blocks);

        let (operator, mut param_blocks) = parse_param(&block.params[5]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            QuotientAndMod {
                id: block.id.clone().into(),
                left,
                right,
                operator,
            }
            .into(),
        );

        blocks
    }
}

impl Block for QuotientAndMod {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        _ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let left = self.left.to_raw_value(memory).unwrap().as_number().unwrap();
        let right = self
            .right
            .to_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();
        let operator = self
            .operator
            .to_raw_value(memory)
            .unwrap()
            .as_string()
            .unwrap();

        let result = match &operator[..] {
            "QUOTIENT" => (left / right).floor(),
            "MOD" => left % right,
            _ => unreachable!(),
        };

        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: Some(Value::Number(result)),
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
