use bevy::prelude::*;

use crate::{
    code::{Code, Codes},
    common::{Id, Ids},
    parser::{RawBlockType, RawObject, RawScript},
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
    objects: &Vec<RawObject>,
    ids: &mut Ids,
) {
    for object in objects {
        let script = RawScript::parse(&object.script).unwrap();
        let mut codes = Vec::new();
        for code in script.0 {
            let event = match code[0].block_type {
                RawBlockType::Event(event) => event,
                _ => continue,
            };

            let mut blocks = Vec::new();
            for raw_block in code.iter().skip(1) {
                blocks.append(&mut raw_block.to_blocks());
            }

            codes.push(Code { event, blocks });
        }
        // info!("{:#?}", codes);

        let id = Id::from_str(&object.id);
        let entity = commands
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
            .insert(id.clone())
            .insert(ObjectType::Sprite)
            .insert(Codes(codes))
            .id();

        ids.insert(id, entity);
    }
}
