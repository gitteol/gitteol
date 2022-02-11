use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::{
    blocks::{functions, Block, BlockType, Value},
    event::EventType,
    object::Object,
    variable::Variable,
    Id,
};

#[derive(Component, Debug)]
pub(crate) struct Code {
    pub(crate) event: EventType,
    pub(crate) blocks: Vec<Block>,
}

#[derive(Debug)]
pub(crate) struct Memory(HashMap<String, Value>);
impl Memory {
    fn new() -> Self {
        Memory(HashMap::new())
    }

    fn format_key(block_id: &Id, label: &str) -> String {
        format!("{}_{}", block_id.0, label)
    }

    pub(crate) fn insert(&mut self, block_id: &Id, label: &str, value: Value) -> Option<Value> {
        self.0.insert(Self::format_key(block_id, label), value)
    }

    pub(crate) fn get(&self, block_id: &Id, label: &str) -> Option<&Value> {
        self.0.get(&Self::format_key(block_id, label))
    }

    pub(crate) fn entry(
        &mut self,
        block_id: &Id,
        label: &str,
    ) -> std::collections::hash_map::Entry<String, Value> {
        self.0.entry(Self::format_key(block_id, label))
    }

    pub(crate) fn remove(&mut self, block_id: &Id, label: &str) -> Option<Value> {
        self.0.remove(&Self::format_key(block_id, label))
    }
}

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
            memory: Memory::new(),
        }
    }
}

pub(crate) struct Queue(pub(crate) VecDeque<CodeRunner>);

pub(crate) fn execute_code(
    mut queue: ResMut<Queue>,
    time: Res<Time>,
    mut transforms: Query<(&mut Transform, &Id), With<Object>>,
    mut variables: Query<&mut Variable>,
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
                BlockType::MoveDirection => functions::move_direction(
                    pointer,
                    &block.args,
                    &memory,
                    &mut transform.translation,
                ),
                BlockType::RepeatBasic => functions::repeat_basic(
                    pointer,
                    &block.args,
                    &block.extra,
                    &block.id,
                    &mut memory,
                ),
                BlockType::RepeatBasicEnd => functions::repeat_basic_end(pointer, &block.extra),
                BlockType::LengthOfString => {
                    functions::length_of_string(pointer, &block.args, &memory)
                }
                BlockType::WaitSecond => {
                    functions::wait_second(pointer, &block.args, &block.id, &mut memory, &time)
                }
                BlockType::SetVariable => {
                    functions::set_variable(pointer, &block.args, &memory, &mut variables)
                }
                BlockType::GetVariable => {
                    functions::get_variable(pointer, &block.args, &memory, &variables)
                }
            };
            pointer = block_return.pointer;
            if let Some(return_value) = block_return.return_value {
                memory.insert(&block.id, "return_value", return_value);
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
