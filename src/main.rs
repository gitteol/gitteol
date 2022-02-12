use std::collections::VecDeque;

use bevy::{core::FixedTimestep, prelude::*};

mod blocks;
mod code;
mod event;
mod object;
mod parser;
mod variable;

use code::Queue;
use event::{Event, EventType};
use serde::Deserialize;
use variable::{spawn_variable, Variable, VariableType};

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Id(String);

impl Id {
    fn from_str(id: &str) -> Id {
        Id(id.to_string())
    }
}

#[derive(Component, Clone)]
struct LocalPos(f32, f32);
impl LocalPos {
    fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
    fn to_variable_pos(&self) -> (f32, f32) {
        (self.0 + 240.0, self.1 + 135.0 - 9.0)
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut events: EventWriter<Event>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let project = parser::parse().unwrap();
    parser::spawn_objects(&mut commands, &asset_server, &project);

    let parent_ui = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .id();
    let font = asset_server.load("fonts/NanumGothic.ttf");
    for raw_variable in project.variables {
        spawn_variable(
            &mut commands,
            font.clone(),
            parent_ui,
            raw_variable.to_variable(),
        );
    }

    events.send(Event {
        event_type: EventType::WhenRunButtonClick,
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "GitTeol".to_string(),
            width: 480.0,
            height: 270.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Queue(VecDeque::new()))
        .add_plugins(DefaultPlugins)
        .add_event::<Event>()
        .add_startup_system(setup)
        .add_system(event::event_listener)
        .add_system(code::execute_code.with_run_criteria(FixedTimestep::step(1.0 / 60.0)))
        .add_system(variable::variable_ui_system)
        .run();
}
