use std::str::FromStr;

use bevy::prelude::*;

use crate::{
    blocks::BlockType,
    code::{Code, Codes},
    common::{Id, Ids},
    event::EventType,
    PROJECT_FILE,
};

#[derive(Component)]
pub(crate) struct Object;

#[derive(Component)]
pub(crate) enum ObjectType {
    Sprite,
}

pub(crate) fn spawn_objects(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    objects: &Vec<dotent::project::object::Object>,
    ids: &mut Ids,
) {
    for object in objects {
        let script = &object.script;
        let mut codes = Vec::new();
        for code in &script.0 {
            let event = match EventType::from_str(&code[0].block_type) {
                Ok(event) => event,
                Err(_) => continue,
            };

            let mut blocks = Vec::new();
            for raw_block in code.iter().skip(1) {
                if let Ok(block_type) = BlockType::from_str(&raw_block.block_type) {
                    let mut block = block_type.build(raw_block);
                    blocks.append(&mut block);
                }
            }

            codes.push(Code { event, blocks });
        }

        let texture = match &object.sprite.pictures[0].filename {
            Some(f) => format!("{}#{}", PROJECT_FILE, f),
            None => "entrybot1.png".to_string(),
        };

        info!("{}", texture);

        let id: Id = object.id.clone().into();
        let entity = commands
            .spawn(SpriteBundle {
                texture: asset_server.load(&texture),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::new(0.315, 0.315, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Object)
            .insert(id.clone())
            .insert(ObjectType::Sprite)
            .insert(Codes(codes))
            .id();

        ids.insert(id, entity);
    }
}
