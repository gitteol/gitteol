use bevy::asset::{AssetLoader, BoxedFuture, LoadedAsset};
use bevy::prelude::*;

use crate::AppState;
use crate::{project::RawProject, ProjectData};

#[derive(Default)]
pub struct ProjectAssetLoader;

impl AssetLoader for ProjectAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let project = serde_json::from_slice::<RawProject>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(project));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

pub(crate) fn setup_asset(asset_server: Res<AssetServer>, mut project_data: ResMut<ProjectData>) {
    project_data.handle = asset_server.load("project.json");
}

pub(crate) fn check_asset_loading(
    project_assets: Res<Assets<RawProject>>,
    project_data: ResMut<ProjectData>,
    mut state: ResMut<State<AppState>>,
) {
    let project = project_assets.get(&project_data.handle);
    if project.is_some() {
        state.set(AppState::MainApp).unwrap();
    };
}
