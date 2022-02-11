use bevy::prelude::*;

use crate::{
    code::{Code, CodeRunner, Queue},
    object::Object,
    Id,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EventType {
    WhenRunButtonClick,
}

pub(crate) struct Event {
    pub(crate) event_type: EventType,
}

pub(crate) fn event_listener(
    mut events: EventReader<Event>,
    mut queue: ResMut<Queue>,
    code: Query<(&Code, &Id), With<Object>>,
) {
    let events: Vec<&EventType> = events.iter().map(|e| &e.event_type).collect();
    code.iter()
        .filter(|(Code { event: e, .. }, _)| events.contains(&e))
        .for_each(|(code, id)| {
            queue
                .0
                .push_back(CodeRunner::new(code.blocks.clone(), id.clone()))
        });
}
