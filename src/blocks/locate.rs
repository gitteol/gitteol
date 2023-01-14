use crate::common::Id;

use super::{parse_param, Block, BlockVec};

#[derive(Clone)]
pub(crate) struct Locate {
    id: Id,
    target: String,
}

impl Locate {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (target, _) = parse_param(&block.params[0]).unwrap();
        let target = target.as_string().unwrap();

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
        _memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let target = &self.target;

        let translation = match &target[..] {
            "mouse" => ctx.mouse.pos,
            _ => {
                let id = Id::from_str(target);
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
