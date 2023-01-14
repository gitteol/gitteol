use crate::common::Id;

use super::{parse_param, Block, BlockReturn, BlockVec, Value};

#[derive(Clone)]
pub(crate) struct CoordinateObject {
    id: Id,
    target: String,
    coordinate: String,
}

impl CoordinateObject {
    pub(crate) fn build(block: &dotent::project::script::Block) -> BlockVec {
        let mut blocks = Vec::new();

        let (target, _) = parse_param(&block.params[1]).unwrap();
        let target = target.as_string().unwrap();

        let (coordinate, _) = parse_param(&block.params[3]).unwrap();
        let coordinate = coordinate.as_string().unwrap();

        blocks.push(
            CoordinateObject {
                id: block.id.clone().into(),
                target,
                coordinate,
            }
            .into(),
        );

        blocks
    }
}

impl Block for CoordinateObject {
    fn run(
        &self,
        pointer: usize,
        _memory: &mut crate::code::Memory,
        ctx: &mut crate::code::Context,
    ) -> super::BlockReturn {
        let target = &self.target;
        let coordinate = &self.coordinate;

        let target_entity = match &target[..] {
            "self" => ctx.owner,
            _ => {
                let id = Id::from_str(target);
                ctx.ids.get(&id).unwrap()
            }
        };
        let target = ctx.objects.get(*target_entity).unwrap();

        let result = match &coordinate[..] {
            "x" => target.translation.x,
            "y" => target.translation.y,
            "rotation" => unimplemented!(),
            "direction" => unimplemented!(),
            "size" => unimplemented!(),
            "picture_index" => unimplemented!(),
            "picture_name" => unimplemented!(),
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
