use bevy::prelude::*;

use crate::{
    code::{CodeRunner, Codes, Queue},
    common::Id,
    object::Object,
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
    codes_query: Query<(&Codes, &Id), With<Object>>,
) {
    for event in events.iter() {
        for (codes, id) in codes_query.iter() {
            for code in &codes.0 {
                if code.event == event.event_type {
                    queue
                        .0
                        .push_back(CodeRunner::new(code.blocks.clone(), id.clone()));
                }
            }
        }
    }
}
