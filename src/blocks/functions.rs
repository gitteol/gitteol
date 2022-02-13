use bevy::prelude::*;

use crate::{
    code::Memory,
    common::{Id, Ids},
    variable::Variable,
};

use super::{BlockReturn, Value};

type Args = [Value];

pub(crate) fn move_direction(
    pointer: usize,
    args: &Args,
    memory: &Memory,
    translation: &mut Vec3,
) -> BlockReturn {
    let amount = args[0].to_raw_value(memory).unwrap().as_number().unwrap();
    translation.x += amount;

    BlockReturn::basic(pointer)
}

pub(crate) fn wait_second(
    pointer: usize,
    args: &Args,
    block_id: &Id,
    memory: &mut Memory,
    time: &Res<Time>,
) -> BlockReturn {
    let second = args[0].to_raw_value(memory).unwrap().as_number().unwrap();

    let delta = memory
        .entry(block_id, "delta")
        .or_insert(Value::Number(0.0))
        .as_number_mut()
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
    extra: &Args,
    block_id: &Id,
    memory: &mut Memory,
) -> BlockReturn {
    let iter_num = args[0].to_raw_value(memory).unwrap().as_number().unwrap();
    let count = memory
        .entry(block_id, "count")
        .or_insert(Value::Number(0.0))
        .as_number_mut()
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
            pointer: pointer + (extra[0].as_number().unwrap() as usize) + 2,
            is_continue: false,
            return_value: None,
        }
    }
}

pub(crate) fn repeat_basic_end(pointer: usize, extra: &Args) -> BlockReturn {
    let length = extra[0].as_number().unwrap();
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
        .as_string()
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
    ids: &Ids,
) -> BlockReturn {
    let variable_id = args[0].as_string().unwrap();
    let variable_entity = ids.get(&Id::from_str(&variable_id)).unwrap();
    let value = args[1].to_raw_value(memory).unwrap();

    let mut variable = variables.get_mut(*variable_entity).unwrap();

    variable.value = value.clone();

    BlockReturn::basic(pointer)
}

pub(crate) fn get_variable(
    pointer: usize,
    args: &Args,
    memory: &Memory,
    variables: &Query<&mut Variable>,
    ids: &Ids,
) -> BlockReturn {
    let variable_id = args[0].to_raw_value(memory).unwrap().as_string().unwrap();
    let variable_entity = ids.get(&Id::from_str(&variable_id)).unwrap();
    let variable = variables.get(*variable_entity).unwrap();

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
        return_value: Some(variable.value.clone()),
    }
}
