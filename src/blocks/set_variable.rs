use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct SetVariable {
    id: Id,
    variable_id: String,
    value: Value,
}
impl Block for SetVariable {
    fn run(&self, pointer: usize, memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let variable_entity = res.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let value = self.value.to_raw_value(memory).unwrap();

        let mut variable = res.variables.get_mut(*variable_entity).unwrap();

        variable.value = value.clone();

        BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl SetVariable {
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let variable_id = block.params[0]
            .to_arg(&mut blocks)
            .unwrap()
            .as_string()
            .unwrap();
        let value = block.params[1].to_arg(&mut blocks).unwrap();
        blocks.push(Box::new(SetVariable {
            id: Id::from_str(&block.id),
            variable_id,
            value,
        }));
        blocks
    }
}
