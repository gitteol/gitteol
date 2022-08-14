use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct ChangeVariable {
    id: Id,
    variable_id: String,
    value: Value,
}
impl Block for ChangeVariable {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let variable_entity = res.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let value = self.value.to_raw_value(memory).unwrap();

        let mut variable = res.variables.get_mut(*variable_entity).unwrap();

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
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let variable_id = block.params[0]
            .to_arg(&mut blocks)
            .unwrap()
            .as_string()
            .unwrap();
        let value = block.params[1].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(ChangeVariable {
            id: Id::from_str(&block.id),
            variable_id,
            value,
        }));
        blocks
    }
}
