use bevy::prelude::*;

use crate::{Id, LocalPos};

#[derive(Clone)]
pub(crate) enum VariableType {
    Normal,
    Timer,
    Answer,
}

#[derive(Component, Clone)]
pub(crate) struct Variable {
    pub(crate) id: Id,
    pub(crate) variable_type: VariableType,
    pub(crate) name: String,
    pub(crate) value: String,
    pub(crate) visible: bool,
    pub(crate) pos: LocalPos,
}

#[derive(Component)]
pub(crate) enum VariableUiType {
    Container,
    Text,
}

#[derive(Component)]
pub(crate) struct VariableUi(Entity);

pub(crate) fn spawn_variable(
    commands: &mut Commands,
    font: Handle<Font>,
    parent_ui: Entity,
    variable: Variable,
) {
    let name = variable.name.clone();
    let position = variable.pos.to_variable_pos();

    let variable_entity = commands.spawn().insert(variable).id();

    commands
        .spawn()
        .insert(VariableUi(variable_entity))
        .insert(VariableUiType::Container)
        .insert(Parent(parent_ui))
        .insert_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(position.0),
                    top: Val::Px(position.1),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::rgb(0.27, 0.53, 0.47).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(3.0)),
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: name,
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: " | ".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    color: Color::rgb(0.56, 0.76, 0.72),
                                },
                            },
                            TextSection {
                                value: "0".to_string(),
                                style: TextStyle {
                                    font: font.clone(),
                                    font_size: 15.0,
                                    color: Color::WHITE,
                                },
                            },
                        ],
                        alignment: TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    },
                    ..Default::default()
                })
                .insert(VariableUiType::Text);
        });
}

pub(crate) fn variable_ui_system(
    uis: Query<(&VariableUi, &Children)>,
    variables: Query<&Variable>,
    mut texts: Query<&mut Text, With<VariableUiType>>,
) {
    for (ui, children) in uis.iter() {
        if let Ok(variable) = variables.get(ui.0) {
            if let Ok(mut text) = texts.get_mut(children[0]) {
                text.sections[2].value = variable.value.clone();
            }
        };
    }
}
