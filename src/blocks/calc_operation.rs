use crate::common::Id;

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct CalcOperation {
    id: Id,
    value: Value,
    operator: String,
}

impl CalcOperation {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (value, mut param_blocks) = parse_param(&block.params[1]).unwrap();
        blocks.append(&mut param_blocks);

        let (operator, _) = parse_param(&block.params[3]).unwrap();
        let operator = operator.as_string().unwrap();

        blocks.push(
            CalcOperation {
                id: block.id.clone().into(),
                value,
                operator,
            }
            .into(),
        );

        blocks
    }
}

impl Block for CalcOperation {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        _ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let value = self
            .value
            .take_raw_value(memory)
            .unwrap()
            .as_number()
            .unwrap();

        let result = match &self.operator[..] {
            "square" => value.powi(2),
            "root" => value.sqrt(),
            "sin" => value.to_radians().sin(),
            "cos" => value.to_radians().cos(),
            "tan" => value.to_radians().tan(),
            "asin_radian" => value.asin().to_degrees(),
            "acos_radian" => value.acos().to_degrees(),
            "atan_radian" => value.atan().to_degrees(),
            "log" => value.log10(),
            "ln" => value.ln(),
            "unnatural" => {
                let unnatural = value - value.round();
                if value < 0.0 {
                    1.0 - unnatural
                } else {
                    unnatural
                }
            }
            "floor" => value.floor(),
            "ceil" => value.ceil(),
            "round" => value.round(),
            "factorial" => value.fract(),
            "abs" => value.abs(),
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
