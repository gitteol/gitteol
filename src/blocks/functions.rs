use bevy::prelude::*;

use crate::{code::Memory, Id};

use super::{BlockReturn, Value};

type Args = [Value];
pub(crate) fn move_direction(pointer: usize, args: &Args, memory: &Memory, translation: &mut Vec3) -> BlockReturn {
    let amount = match args[0].memory(memory) {
        Some(i) => i.number(),
        None => args[0].number(),
    }
    .unwrap();
    translation.x += amount;

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: None,
    }
}

pub(crate) fn repeat_basic(
    pointer: usize,
    args: &Args,
    block_id: &Id,
    memory: &mut Memory,
) -> BlockReturn {
    let iter_num = match args[0].memory(memory) {
        Some(i) => i.number(),
        None => args[0].number(),
    }
    .unwrap();
    let memory_key = format!("{}_count", block_id.0);
    let count = memory
        .entry(memory_key.clone())
        .or_insert(Value::Number(0.0))
        .number_mut()
        .unwrap();
    if *count < iter_num {
        *count += 1.0;
        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: None,
        }
    } else {
        memory.remove(&memory_key);
        BlockReturn {
            pointer: pointer + (args[1].number().unwrap() as usize) + 2,
            is_continue: false,
            return_value: None,
        }
    }
}

pub(crate) fn repeat_basic_end(pointer: usize, args: &Args) -> BlockReturn {
    let length = args[0].number().unwrap();
    BlockReturn {
        pointer: pointer - (length as usize) - 1,
        is_continue: true,
        return_value: None,
    }
}

pub(crate) fn length_of_string(pointer: usize, args: &Args) -> BlockReturn {
    let length = args[0].string().unwrap().chars().count();

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: Some(Value::Number(length as f32)),
    }
}
