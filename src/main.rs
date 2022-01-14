use std::collections::VecDeque;

use bevy::{core::FixedTimestep, prelude::*};

mod blocks;
mod code;
mod event;
mod object;

use blocks::{
    state::{BlockState, RepeatEndState, RepeatState},
    Block, BlockType, Value,
};
use code::{Code, Queue};
use event::{Event, EventType};
use object::{Object, ObjectType};

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Id(String);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut events: EventWriter<Event>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("entrybot1.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(0.315, 0.315, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Object)
        .insert(Id("tund".to_string()))
        .insert(ObjectType::Sprite)
        .insert(Code {
            event: EventType::WhenRunButtonClick,
            blocks: vec![
                Block {
                    block_type: BlockType::RepeatBasic,
                    args: vec![Value::Number(10.0)],
                    state: BlockState::RepeatBasic(RepeatState {
                        length: 1,
                        ..Default::default()
                    }),
                },
                Block {
                    block_type: BlockType::MoveDirection,
                    args: vec![Value::Number(10.0)],
                    state: BlockState::None,
                },
                Block {
                    block_type: BlockType::RepeatBasicEnd,
                    args: vec![],
                    state: BlockState::RepeatBasicEnd(RepeatEndState { length: 1 }),
                },
            ],
        });

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
        .run();
}
