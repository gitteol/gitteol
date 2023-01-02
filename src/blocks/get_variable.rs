use crate::{
    code::{Context, Memory},
    common::Id,
};

use super::{parse_param, Block, BlockReturn, BlockVec};

#[derive(Clone)]
pub(crate) struct GetVariable {
    id: Id,
    variable_id: String,
}
impl Block for GetVariable {
    fn run(&self, pointer: usize, _memory: &mut Memory, ctx: &mut Context) -> BlockReturn {
        let variable_entity = ctx.ids.get(&Id::from_str(&self.variable_id)).unwrap();
        let variable = ctx.variables.get(*variable_entity).unwrap();

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
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks: BlockVec = Vec::new();

        let (variable_id, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        let variable_id = variable_id.as_string().unwrap();

        blocks.push(
            GetVariable {
                id: block.id.clone().into(),
                variable_id,
            }
            .into(),
        );
        blocks
    }
}
