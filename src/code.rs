use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    blocks::{functions, Block, BlockType},
    event::EventType,
    object::Object,
    Id,
};

#[derive(Component)]
pub(crate) struct Code {
    pub(crate) event: EventType,
    pub(crate) blocks: Vec<Block>,
}

#[derive(Debug)]
pub(crate) struct CodeRunner {
    pub(crate) code: Vec<Block>,
    pub(crate) pointer: usize,
    pub(crate) owner: Id,
}

pub(crate) struct Queue(pub(crate) VecDeque<CodeRunner>);

pub(crate) fn execute_code(
    mut queue: ResMut<Queue>,
    mut transforms: Query<(&mut Transform, &Id), With<Object>>,
) {
    let mut new_queue: VecDeque<CodeRunner> = VecDeque::new();

    while !queue.0.is_empty() {
        let CodeRunner {
            mut code,
            mut pointer,
            owner,
        } = queue.0.pop_front().unwrap();

        let (mut transform, _) = transforms.iter_mut().find(|(_, id)| **id == owner).unwrap();

        while let Some(block) = code.get_mut(pointer) {
            let block_return = match block.block_type {
                BlockType::MoveDirection => {
                    functions::move_direction(pointer, &block.args, &mut transform.translation)
                }
                BlockType::RepeatBasic => {
                    functions::repeat_basic(pointer, &block.args, &mut block.state)
                }
                BlockType::RepeatBasicEnd => functions::repeat_basic_end(pointer, &mut block.state),
            };
            pointer = block_return.pointer;
            if block_return.is_continue {
                new_queue.push_back(CodeRunner {
                    code,
                    pointer,
                    owner,
                });
                break;
            }
        }
    }
    queue.0 = new_queue;
}
