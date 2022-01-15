use std::collections::VecDeque;

use bevy::{core::FixedTimestep, prelude::*};

mod blocks;
mod code;
mod event;
mod object;

use blocks::{Block, BlockType, Value};
use code::{Code, Queue};
use event::{Event, EventType};
use object::{Object, ObjectType};

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
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
                    id: Id("c5q3".to_string()),
                    block_type: BlockType::LengthOfString,
                    args: vec![Value::String("안녕, 엔트리!".to_string())],
                },
                Block {
                    id: Id("c5q1".to_string()),
                    block_type: BlockType::RepeatBasic,
                    args: vec![
                        Value::Memory((Id("c5q3".to_string()), "return_value".to_string())),
                        Value::Number(1.0),
                    ],
                },
                Block {
                    id: Id("niob".to_string()),
                    block_type: BlockType::MoveDirection,
                    args: vec![Value::Number(10.0)],
                },
                Block {
                    id: Id("c5q1".to_string()),
                    block_type: BlockType::RepeatBasicEnd,
                    args: vec![Value::Number(1.0)],
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
