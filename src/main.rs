use std::collections::VecDeque;

use bevy::{core::FixedTimestep, prelude::*};

mod blocks;
mod code;
mod event;
mod object;
mod variable;

use blocks::{Block, BlockType, Value};
use code::{Code, Queue};
use event::{Event, EventType};
use object::{Object, ObjectType};
use variable::{spawn_variable, Variable, VariableType};

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
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
        .insert(Id::from_str("tund"))
        .insert(ObjectType::Sprite)
        .insert(Code {
            event: EventType::WhenRunButtonClick,
            blocks: vec![
                Block {
                    id: Id::from_str("ai3d"),
                    block_type: BlockType::WaitSecond,
                    args: vec![Value::Number(2.0)],
                    extra: vec![],
                },
                Block {
                    id: Id::from_str("niod"),
                    block_type: BlockType::SetVariable,
                    args: vec![
                        Value::String("ie7y".to_string()),
                        Value::String("20".to_string()),
                    ],
                    extra: vec![],
                },
                Block {
                    id: Id::from_str("15qa"),
                    block_type: BlockType::GetVariable,
                    args: vec![Value::String("ie7y".to_string())],
                    extra: vec![],
                },
                Block {
                    id: Id::from_str("c5q1"),
                    block_type: BlockType::RepeatBasic,
                    args: vec![Value::Memory((
                        Id::from_str("15qa"),
                        "return_value".to_string(),
                    ))],
                    extra: vec![Value::Number(2.0)],
                },
                Block {
                    id: Id::from_str("niob"),
                    block_type: BlockType::MoveDirection,
                    args: vec![Value::Number(10.0)],
                    extra: vec![],
                },
                Block {
                    id: Id::from_str("c5q1"),
                    block_type: BlockType::RepeatBasicEnd,
                    args: vec![],
                    extra: vec![Value::Number(2.0)],
                },
            ],
        });

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
    spawn_variable(
        &mut commands,
        font,
        parent_ui,
        Variable {
            id: Id::from_str("ie7y"),
            variable_type: VariableType::Normal,
            name: "변수2".to_string(),
            value: "014".to_string(),
            visible: true,
            pos: LocalPos::new(-230.0, -105.0),
        },
    );

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
