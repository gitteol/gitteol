use bevy::prelude::*;
use serde_json::Result;

pub(crate) mod types;

use crate::{
    code::{Code, Codes},
    object::{Object, ObjectType},
    Id,
};

use self::types::{RawBlockType, RawProject, RawScript};

pub(crate) fn parse() -> Result<RawProject> {
    let data = include_str!("../../assets/project.json");

    serde_json::from_str(data)
}

pub(crate) fn spawn_entities(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    project: RawProject,
) {
    for object in project.objects {
        let script: RawScript = serde_json::from_str(&object.script).unwrap();
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
            .insert(Id(object.id))
            .insert(ObjectType::Sprite)
            .insert(Codes(codes));
    }
}
