use std::collections::VecDeque;

use asset::{EntryAssetLoader, EntryFile, EntryProject};
use bevy::{prelude::*, time::FixedTimestep};

mod asset;
mod blocks;
mod code;
mod common;
mod event;
mod object;
mod variable;

use code::Queue;
use common::Ids;
use event::{Event, EventType};
use object::spawn_objects;
use variable::spawn_variable;

const PROJECT_FILE: &str = "project.ent";
const WINDOW_ASPECT_RATIO: f32 = 480.0 / 270.0;
const ENTRY_WIDTH: f32 = 480.0;
const WINDOW_SIZE: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    MainApp,
}

#[derive(Default, Resource)]
struct EntryFileData {
    handle: Handle<EntryFile>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<Event>,
    project_assets: Res<Assets<EntryProject>>,
    mut ids: ResMut<Ids>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn(Camera2dBundle::default());

    let project = &project_assets
        .get(&asset_server.load(format!("{}#project", PROJECT_FILE)))
        .unwrap()
        .0;

    let window = windows.primary_mut();
    window.set_title(format!("{}", project.name));

    spawn_objects(&mut commands, &asset_server, &project.objects, &mut ids);

    let font = asset_server.load("fonts/NanumGothic.ttf");

    let mut variable_ui_children = Vec::new();
    for raw_variable in &project.variables {
        variable_ui_children.push(spawn_variable(
            &mut commands,
            font.clone(),
            raw_variable.clone().into(),
            &mut ids,
        ))
    }

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .push_children(&variable_ui_children);

    events.send(Event {
        event_type: EventType::WhenRunButtonClick,
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(Queue(VecDeque::new()))
        .init_resource::<EntryFileData>()
        .insert_resource(Ids::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "깃털".to_string(),
                width: ENTRY_WIDTH * WINDOW_SIZE,
                height: ENTRY_WIDTH * WINDOW_SIZE / WINDOW_ASPECT_RATIO,
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(asset::setup_asset))
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(asset::check_asset_loading),
        )
        .add_system_set(SystemSet::on_enter(AppState::MainApp).with_system(setup))
        .add_system_set(
            SystemSet::on_update(AppState::MainApp)
                .with_system(event::event_listener)
                .with_system(variable::variable_ui_system)
                .with_system(object::object_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainApp)
                .with_system(code::execute_code)
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0)),
        )
        .add_event::<Event>()
        .add_asset::<EntryProject>()
        .add_asset::<EntryFile>()
        .init_asset_loader::<EntryAssetLoader>()
        .run();
}
