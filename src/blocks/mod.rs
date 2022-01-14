pub(crate) mod functions;
pub(crate) mod state;

#[derive(Clone, Debug)]
pub(crate) enum BlockType {
    MoveDirection,
    RepeatBasic,
    RepeatBasicEnd,
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    String(String),
    Number(f32),
    Boolean(bool),
}
impl Value {
    fn string(&self) -> Option<&str> {
        match self {
            Self::String(val) => Some(&val[..]),
            _ => None,
        }
    }
    fn number(&self) -> Option<f32> {
        match self {
            Self::Number(val) => Some(*val),
            _ => None,
        }
    }
    fn boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(val) => Some(*val),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Block {
    pub(crate) block_type: BlockType,
    pub(crate) args: Vec<Value>,
    pub(crate) state: state::BlockState,
}

pub(crate) struct BlockReturn {
    pub(crate) pointer: usize,
    pub(crate) is_continue: bool,
}
