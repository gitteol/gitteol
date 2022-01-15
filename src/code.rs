use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::{
    blocks::{functions, Block, BlockType, Value},
    event::EventType,
    object::Object,
    Id,
};

#[derive(Component)]
pub(crate) struct Code {
    pub(crate) event: EventType,
    pub(crate) blocks: Vec<Block>,
}

pub(crate) type Memory = HashMap<String, Value>;
pub(crate) struct CodeRunner {
    code: Vec<Block>,
    pointer: usize,
    owner: Id,
    memory: Memory,
}
impl CodeRunner {
    pub(crate) fn new(code: Vec<Block>, owner: Id) -> Self {
        CodeRunner {
            code,
            pointer: 0,
            owner,
            memory: HashMap::new(),
        }
    }
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
            mut memory,
        } = queue.0.pop_front().unwrap();

        let (mut transform, _) = transforms.iter_mut().find(|(_, id)| **id == owner).unwrap();

        while let Some(block) = code.get_mut(pointer) {
            let block_return = match block.block_type {
                BlockType::MoveDirection => {
                    functions::move_direction(pointer, &block.args, &memory, &mut transform.translation)
                }
                BlockType::RepeatBasic => {
                    functions::repeat_basic(pointer, &block.args, &block.id, &mut memory)
                }
                BlockType::RepeatBasicEnd => functions::repeat_basic_end(pointer, &block.args),
                BlockType::LengthOfString => functions::length_of_string(pointer, &block.args),
            };
            pointer = block_return.pointer;
            if let Some(return_value) = block_return.return_value {
                memory.insert(block.id.0.clone(), return_value);
            }
            info!("pointer: {}, memory: {:?}", pointer, memory);
            if block_return.is_continue {
                new_queue.push_back(CodeRunner {
                    code,
                    pointer,
                    owner,
                    memory,
                });
                break;
            }
        }
    }
    queue.0 = new_queue;
}
