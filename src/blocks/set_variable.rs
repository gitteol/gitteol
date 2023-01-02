use crate::{
    code::{Memory, Resources},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

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
    pub(crate) fn new(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (variable_id, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let (value, mut param_blocks) = parse_param(&block.params[1]).unwrap();
        blocks.append(&mut param_blocks);

        let variable_id = variable_id.as_string().unwrap();

        blocks.push(
            SetVariable {
                id: block.id.clone().into(),
                variable_id,
                value,
            }
            .into(),
        );
        blocks
    }
}
