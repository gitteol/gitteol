use bevy::prelude::*;

use crate::{code::Memory, variable::Variable, Id};

use super::{BlockReturn, Value};

type Args = [Value];

pub(crate) fn move_direction(
    pointer: usize,
    args: &Args,
    memory: &Memory,
    translation: &mut Vec3,
) -> BlockReturn {
    let amount = args[0].to_raw_value(memory).unwrap().number().unwrap();
    translation.x += amount;

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: None,
    }
}

pub(crate) fn wait_second(
    pointer: usize,
    args: &Args,
    block_id: &Id,
    memory: &mut Memory,
    time: &Res<Time>,
) -> BlockReturn {
    let second = args[0].to_raw_value(memory).unwrap().number().unwrap();

    let delta = memory
        .entry(block_id, "delta")
        .or_insert(Value::Number(0.0))
        .number_mut()
        .unwrap();

    *delta += time.delta_seconds();

    if *delta >= second {
        memory.remove(block_id, "delta");
        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
            return_value: None,
        }
    } else {
        BlockReturn {
            pointer,
            is_continue: true,
            return_value: None,
        }
    }
}

pub(crate) fn repeat_basic(
    pointer: usize,
    args: &Args,
    block_id: &Id,
    memory: &mut Memory,
) -> BlockReturn {
    let iter_num = args[0].to_raw_value(memory).unwrap().number().unwrap();
    let count = memory
        .entry(block_id, "count")
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
        memory.remove(block_id, "count");
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

pub(crate) fn length_of_string(pointer: usize, args: &Args, memory: &Memory) -> BlockReturn {
    let length = args[0]
        .to_raw_value(memory)
        .unwrap()
        .string()
        .unwrap()
        .chars()
        .count();

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: Some(Value::Number(length as f32)),
    }
}

pub(crate) fn set_variable(
    pointer: usize,
    args: &Args,
    memory: &Memory,
    variables: &mut Query<&mut Variable>,
) -> BlockReturn {
    let variable_id = args[0].string().unwrap();
    let value = args[1].to_raw_value(memory).unwrap().string().unwrap();

    let mut variable = variables
        .iter_mut()
        .find(|variable| variable.id.0 == variable_id)
        .unwrap();
    variable.value = value.to_string();

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: None,
    }
}

pub(crate) fn get_variable(
    pointer: usize,
    args: &Args,
    memory: &Memory,
    variables: &Query<&mut Variable>,
) -> BlockReturn {
    let variable_id = args[0].string().unwrap();

    let variable = variables
        .iter()
        .find(|variable| variable.id.0 == variable_id)
        .unwrap();
    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: Some(Value::String(variable.value.clone())),
    }
}
