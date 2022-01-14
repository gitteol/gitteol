#[derive(Clone, Debug, Default)]
pub(crate) struct RepeatState {
    pub(crate) count: u32,
    pub(crate) length: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct RepeatEndState {
    pub(crate) length: usize,
}

#[derive(Clone, Debug)]
pub(crate) enum BlockState {
    RepeatBasic(RepeatState),
    RepeatBasicEnd(RepeatEndState),
    None,
}
impl BlockState {
    pub(super) fn repeat_basic(&mut self) -> Option<&mut RepeatState> {
        match self {
            Self::RepeatBasic(state) => Some(state),
            _ => None,
        }
    }
    pub(super) fn repeat_basic_end(&mut self) -> Option<&mut RepeatEndState> {
        match self {
            Self::RepeatBasicEnd(state) => Some(state),
            _ => None,
        }
    }
}
