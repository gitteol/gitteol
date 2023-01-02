use std::collections::VecDeque;

use asset::{EntryAssetLoader, EntryProject};
use bevy::{prelude::*, time::FixedTimestep};

mod asset;
mod blocks;
mod code;
mod common;
mod event;
mod object;
// mod project;
mod variable;

use code::Queue;
use common::Ids;
use event::{Event, EventType};
use object::spawn_objects;
use variable::spawn_variable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    MainApp,
}

#[derive(Default)]
struct ProjectData {
    handle: Handle<EntryProject>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<Event>,
    project_assets: Res<Assets<EntryProject>>,
    project_data: Res<ProjectData>,
    mut ids: ResMut<Ids>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let project = &project_assets.get(&project_data.handle).unwrap().0;

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
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .push_children(&variable_ui_children);

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
        .init_resource::<ProjectData>()
        .insert_resource(Ids::new())
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(asset::setup_asset))
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(asset::check_asset_loading),
        )
        .add_system_set(SystemSet::on_enter(AppState::MainApp).with_system(setup))
        .add_system_set(
            SystemSet::on_update(AppState::MainApp)
                .with_system(event::event_listener)
                .with_system(variable::variable_ui_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainApp)
                .with_system(code::execute_code)
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0)),
        )
        .add_event::<Event>()
        .add_asset::<EntryProject>()
        .init_asset_loader::<EntryAssetLoader>()
        .run();
}
