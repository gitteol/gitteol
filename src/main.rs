use std::collections::VecDeque;

use asset::ProjectAssetLoader;
use bevy::{core::FixedTimestep, prelude::*};

mod asset;
mod blocks;
mod code;
mod common;
mod event;
mod object;
mod project;
mod variable;

use code::Queue;
use common::Ids;
use event::{Event, EventType};
use object::spawn_objects;
use project::RawProject;
use variable::spawn_variable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    MainApp,
}

#[derive(Default)]
struct ProjectData {
    handle: Handle<RawProject>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<Event>,
    project_assets: Res<Assets<RawProject>>,
    project_data: ResMut<ProjectData>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let mut ids = Ids::new();

    let project = project_assets.get(&project_data.handle).unwrap();

    spawn_objects(&mut commands, &asset_server, &project.objects, &mut ids);

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
    for raw_variable in &project.variables {
        spawn_variable(
            &mut commands,
            font.clone(),
            parent_ui,
            raw_variable.to_variable(),
            &mut ids,
        );
    }

    commands.insert_resource(ids);

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
        .add_asset::<RawProject>()
        .init_asset_loader::<ProjectAssetLoader>()
        .run();
}
