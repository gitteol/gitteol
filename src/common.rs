use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub(crate) struct Id(pub(crate) String);

impl Id {
    pub(crate) fn from_str(id: &str) -> Id {
        Id(id.to_string())
    }
}

#[derive(Component, Clone)]
pub(crate) struct LocalPos(f32, f32);
impl LocalPos {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
    pub(crate) fn to_variable_pos(&self) -> (f32, f32) {
        (self.0 + 240.0, self.1 + 135.0 - 9.0)
    }
}

pub(crate) struct Ids(HashMap<Id, Entity>);

impl Ids {
    pub(crate) fn new() -> Ids {
        Ids(HashMap::new())
    }

    pub(crate) fn insert(&mut self, id: Id, entity: Entity) -> Option<Entity> {
        self.0.insert(id, entity)
    }

    pub(crate) fn get(&self, id: &Id) -> Option<&Entity> {
        self.0.get(id)
    }
}
