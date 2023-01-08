use crate::common::Id;

use super::{parse_param, Block, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct Locate {
    id: Id,
    target: Value,
}

impl Locate {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (target, mut param_blocks) = parse_param(&block.params[0]).unwrap();
        blocks.append(&mut param_blocks);

        blocks.push(
            Locate {
                id: block.id.clone().into(),
                target,
            }
            .into(),
        );

        blocks
    }
}

impl Block for Locate {
    fn run(
        &self,
        pointer: usize,
        memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let target = self
            .target
            .to_raw_value(memory)
            .unwrap()
            .as_string()
            .unwrap();

        let translation = match &target[..] {
            "mouse" => ctx.mouse.pos,
            _ => {
                let id = Id::from_str(&target);
                let entity = ctx.ids.get(&id).unwrap();
                let target = ctx.objects.get(*entity).unwrap();
                target.translation.truncate()
            }
        };

        let mut this = ctx.objects.get_mut(*ctx.owner).unwrap();
        this.translation.x = translation.x;
        this.translation.y = translation.y;

        super::BlockReturn::basic(pointer)
    }

    fn get_id(&self) -> &Id {
        &self.id
    }
}
