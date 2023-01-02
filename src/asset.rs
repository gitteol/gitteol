use bevy::asset::{AssetLoader, BoxedFuture, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use dotent::asset::Asset;
use dotent::entry::Entry;

use crate::AppState;
use crate::ProjectData;

#[derive(TypeUuid)]
#[uuid = "fef49495-a238-414e-b08f-8474c9f923ff"]
pub struct EntryProject(pub dotent::project::Project);

#[derive(Default)]
pub struct EntryAssetLoader;

impl AssetLoader for EntryAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            // let project = serde_json::from_slice::<RawProject>(bytes)?;
            // load_context.set_default_asset(LoadedAsset::new(project));

            let entry = Entry::read(bytes)?;
            let project = EntryProject(entry.project().clone());

            load_context.set_default_asset(LoadedAsset::new(project));

            let assets = entry.assets();
            for (key, asset) in assets {
                match asset {
                    Asset::Image(asset) => {
                        // https://github.com/bevyengine/bevy/blob/b027d402e29d7d4f26062051161bc2187f168e80/crates/bevy_render/src/texture/image_texture_loader.rs#L49-L58
                        let dyn_img = Image::from_buffer(
                            asset.data(),
                            ImageType::Extension(asset.ext()),
                            CompressedImageFormats::default(),
                            true,
                        )?;
                        load_context.set_labeled_asset(asset.name(), LoadedAsset::new(dyn_img));
                    }
                    Asset::Sound(_) => continue, // TODO: impl this
                }
            }

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ent"]
    }
}

pub(crate) fn setup_asset(asset_server: Res<AssetServer>, mut project_data: ResMut<ProjectData>) {
    project_data.handle = asset_server.load("project.ent");
}

pub(crate) fn check_asset_loading(
    project_assets: Res<Assets<EntryProject>>,
    project_data: ResMut<ProjectData>,
    mut state: ResMut<State<AppState>>,
) {
    let project = project_assets.get(&project_data.handle);
    if project.is_some() {
        state.set(AppState::MainApp).unwrap();
    };
}
