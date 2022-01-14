use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Object;

#[derive(Component)]
pub(crate) enum ObjectType {
    Sprite,
}
