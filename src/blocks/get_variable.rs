use crate::{
    code::{Memory, Resources},
    common::Id,
    project::RawBlock,
};

use super::{Block, BlockReturn, BlockVec};

#[derive(Clone)]
pub(crate) struct GetVariable {
    id: Id,
    variable_id: String,
}
impl Block for GetVariable {
    fn run(&self, pointer: usize, _memory: &mut Memory, res: &mut Resources) -> BlockReturn {
        let variable_entity = res.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let variable = res.variables.get(*variable_entity).unwrap();

        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: Some(variable.value.clone()),
        }
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
impl GetVariable {
    pub(crate) fn new(block: &RawBlock) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();
        let variable_id = block.params[0]
            .to_arg(&mut blocks)
            .unwrap()
            .as_string()
            .unwrap();
        blocks.push(Box::new(GetVariable {
            id: Id::from_str(&block.id),
            variable_id,
        }));
        blocks
    }
}
