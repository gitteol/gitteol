use std::collections::HashMap;

use bevy::asset::{AssetLoader, BoxedFuture, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use dotent::asset::Asset;
use dotent::entry::Entry;

use crate::{AppState, EntryFileData, PROJECT_FILE};

#[derive(TypeUuid)]
#[uuid = "eaa5b97c-a72e-4cd4-9c29-2708e58fbd61"]
pub struct EntryProject(pub dotent::project::Project);

#[derive(TypeUuid, Default)]
#[uuid = "3a54cf64-1848-4944-bac7-1caa59ccf92c"]
pub struct EntryFile {
    pub project: Handle<EntryProject>,
    pub images: HashMap<String, Handle<Image>>,
}

#[derive(Default)]
pub struct EntryAssetLoader;

impl AssetLoader for EntryAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let entry = Entry::read(bytes)?;
            let project = EntryProject(entry.project().clone());

            let project_handle = load_context
                .set_labeled_asset::<EntryProject>("project", LoadedAsset::new(project));

            let assets = entry.assets();
            let mut images_map = HashMap::new();
            for (key, asset) in assets {
                match asset {
                    Asset::Image(image_data) => {
                        // https://github.com/bevyengine/bevy/blob/b027d402e29d7d4f26062051161bc2187f168e80/crates/bevy_render/src/texture/image_texture_loader.rs#L49-L58
                        let dyn_img = Image::from_buffer(
                            image_data.data(),
                            ImageType::Extension(image_data.ext()),
                            CompressedImageFormats::default(),
                            true,
                        )?;
                        let handle =
                            load_context.set_labeled_asset::<Image>(key, LoadedAsset::new(dyn_img));
                        images_map.insert(key.to_string(), handle);
                    }
                    Asset::Sound(_) => continue, // TODO: impl this
                }
            }

            load_context.set_default_asset(LoadedAsset::new(EntryFile {
                project: project_handle,
                images: images_map,
            }));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ent"]
    }
}

pub(crate) fn setup_asset(asset_server: Res<AssetServer>, mut entry_file: ResMut<EntryFileData>) {
    entry_file.handle = asset_server.load(PROJECT_FILE);
}

pub(crate) fn check_asset_loading(
    entry_file_assets: Res<Assets<EntryFile>>,
    entry_file_data: Res<EntryFileData>,
    mut state: ResMut<State<AppState>>,
) {
    let entry_file = entry_file_assets.get(&entry_file_data.handle);
    if entry_file.is_some() {
        state.set(AppState::MainApp).unwrap();
    };
}
