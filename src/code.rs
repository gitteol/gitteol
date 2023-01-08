use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use crate::{
    blocks::{Block, BlockVec, Value},
    common::{Id, Ids},
    event::EventType,
    mouse::Mouse,
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

pub(crate) struct Context<'a, 'b1, 'c1, 'd1, 'b2, 'c2, 'd2> {
    pub(crate) time: &'a Res<'a, Time>,
    pub(crate) ids: &'a Res<'a, Ids>,
    // pub(crate) this: &'a mut Object,
    pub(crate) owner: &'a Entity,
    pub(crate) objects: &'a mut Query<'b1, 'c1, &'d1 mut Object>,
    pub(crate) variables: &'a mut Query<'b2, 'c2, &'d2 mut Variable>,
    pub(crate) mouse: &'a Res<'a, Mouse>,
}

#[derive(Resource)]
pub(crate) struct Queue(pub(crate) VecDeque<CodeRunner>);

pub(crate) fn execute_code(
    mut queue: ResMut<Queue>,
    time: Res<Time>,
    ids: Res<Ids>,
    mut objects: Query<&mut Object>,
    mut variables: Query<&mut Variable>,
    mouse: Res<Mouse>,
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

        let mut ctx = Context {
            time: &time,
            ids: &ids,
            owner: owner_entity,
            objects: &mut objects,
            variables: &mut variables,
            mouse: &mouse,
        };

        while let Some(block) = code.get_mut(pointer) {
            let block_return = block.run(pointer, &mut memory, &mut ctx);
            pointer = block_return.pointer;
            if let Some(return_value) = block_return.return_value {
                memory.insert(block.get_id(), "return_value", return_value);
            }

            // info!(
            //     "OBJECT: {:?}, CODE: {}, POINTER: {} MEMORY: {:#?}",
            //     owner_entity,
            //     code.first()
            //         .map(|c| c.get_id().0.clone())
            //         .unwrap_or_else(|| "None".to_string()),
            //     pointer,
            //     memory.0
            // );

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
