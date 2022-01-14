use bevy::prelude::*;

use super::{state, BlockReturn, Value};

type Args = [Value];
pub(crate) fn move_direction(pointer: usize, args: &Args, translation: &mut Vec3) -> BlockReturn {
    translation.x += args[0].number().unwrap();

    BlockReturn {
        pointer: pointer + 1,
        is_continue: false,
    }
}

pub(crate) fn repeat_basic(
    pointer: usize,
    args: &Args,
    state: &mut state::BlockState,
) -> BlockReturn {
    let state = state.repeat_basic().unwrap();
    if state.count < args[0].number().unwrap() as u32 {
        state.count += 1;
        BlockReturn {
            pointer: pointer + 1,
            is_continue: false,
        }
    } else {
        BlockReturn {
            pointer: pointer + state.length + 2,
            is_continue: false,
        }
    }
}

pub(crate) fn repeat_basic_end(pointer: usize, state: &mut state::BlockState) -> BlockReturn {
    let state = state.repeat_basic_end().unwrap();
    BlockReturn {
        pointer: pointer - state.length - 1,
        is_continue: true,
    }
}
