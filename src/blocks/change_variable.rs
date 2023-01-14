use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct ChangeVariable {
    id: Id,
    variable_id: String,
    value: Value,
}
impl Block for ChangeVariable {
    fn run(&self, pointer: usize, memory: &mut Memory, ctx: &mut Context) -> BlockReturn {
        let variable_entity = ctx.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let value = self.value.take_raw_value(memory).unwrap();

        let mut variable = ctx.variables.get_mut(*variable_entity).unwrap();

        if variable.value.as_number().is_ok() && value.as_number().is_ok() {
            *variable.value.as_number_mut().unwrap() += value.as_number().unwrap();
        } else {
            variable.value = Value::String(format!(
                "{}{}",
                variable.value.as_string().unwrap(),
                value.as_string().unwrap()
            ))
        }

        BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl ChangeVariable {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (variable_id, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let (value, mut param_blocks) = parse_param(&block.params[1]).unwrap();
        blocks.append(&mut param_blocks);

        let variable_id = variable_id.as_string().unwrap();

        blocks.push(
            ChangeVariable {
                id: block.id.clone().into(),
                variable_id,
                value,
            }
            .into(),
        );
        blocks
    }
}
