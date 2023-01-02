use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::{
    blocks::{Block, BlockVec, Value},
    common::{Id, Ids},
    event::EventType,
    object::Object,
    variable::Variable,
};

#[derive(Component)]
pub(crate) struct Code {
    pub(crate) event: EventType,
    pub(crate) blocks: BlockVec,
}

#[derive(Component)]
pub(crate) struct Codes(pub(crate) Vec<Code>);

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
    code: BlockVec,
    pointer: usize,
    owner: Id,
    memory: Memory,
}
impl CodeRunner {
    pub(crate) fn new(code: BlockVec, owner: Id) -> Self {
        CodeRunner {
            code,
            pointer: 0,
            owner,
            memory: Memory::new(),
        }
    }
}

pub(crate) struct Context<'a, 'b, 'c, 'd> {
    pub(crate) time: &'a Res<'a, Time>,
    pub(crate) ids: &'a Res<'a, Ids>,
    pub(crate) object: &'a mut Object,
    pub(crate) variables: &'a mut Query<'b, 'c, &'d mut Variable>,
}

#[derive(Resource)]
pub(crate) struct Queue(pub(crate) VecDeque<CodeRunner>);

pub(crate) fn execute_code(
    mut queue: ResMut<Queue>,
    time: Res<Time>,
    ids: Res<Ids>,
    mut objects: Query<&mut Object>,
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

        let owner_entity = ids.get(&owner).unwrap();

        let mut this_object = objects.get_mut(*owner_entity).unwrap();

        let mut ctx = Context {
            time: &time,
            ids: &ids,
            object: &mut this_object,
            variables: &mut variables,
        };

        while let Some(block) = code.get_mut(pointer) {
            let block_return = block.run(pointer, &mut memory, &mut ctx);
            pointer = block_return.pointer;
            if let Some(return_value) = block_return.return_value {
                memory.insert(block.get_id(), "return_value", return_value);
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
