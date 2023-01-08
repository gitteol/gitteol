use bevy::prelude::*;
use dotent::project::variable::VariableType;

use crate::{
    blocks::Value,
    common::{Id, Ids, LocalPos},
};

fn get_variable_color(variable_type: &VariableType) -> Color {
    match variable_type {
        VariableType::Variable | VariableType::Slide | VariableType::List => {
            Color::rgb(0.36, 0.50, 0.97)
        }
        VariableType::Timer => Color::rgb(0.92, 0.70, 0.26),
        VariableType::Answer => Color::rgb(0.90, 0.51, 0.92),
    }
}

#[derive(Component, Clone)]
pub(crate) struct Variable {
    pub(crate) id: Id,
    pub(crate) variable_type: dotent::project::variable::VariableType,
    pub(crate) name: String,
    pub(crate) value: Value,
    pub(crate) visible: bool,
    pub(crate) pos: LocalPos,
}
impl From<dotent::project::variable::Variable> for Variable {
    fn from(value: dotent::project::variable::Variable) -> Self {
        Variable {
            id: value.id.into(),
            variable_type: value.variable_type,
            name: value.name,
            value: value.value.into(),
            visible: value.visible,
            pos: LocalPos::new(value.x, value.y),
        }
    }
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
    variable: Variable,
    ids: &mut Ids,
) -> Entity {
    let id = variable.id.clone();
    let name = variable.name.clone();
    let position = variable.pos.to_variable_pos();
    let color = get_variable_color(&variable.variable_type);

    let variable_entity = commands.spawn_empty().insert(variable).id();

    ids.insert(id, variable_entity);

    commands
        .spawn_empty()
        .insert(VariableUi(variable_entity))
        .insert(VariableUiType::Container)
        .insert(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(position.0),
                    top: Val::Px(position.1),
                    ..Default::default()
                },
                display: Display::Flex,
                ..Default::default()
            },
            background_color: color.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(3.0)),
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
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
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
        })
        .id()
}

pub(crate) fn variable_ui_system(
    mut uis: Query<(&VariableUi, &mut Style, &Children)>,
    variables: Query<&Variable>,
    mut texts: Query<&mut Text, With<VariableUiType>>,
) {
    for (ui, mut style, children) in uis.iter_mut() {
        if let Ok(variable) = variables.get(ui.0) {
            if let Ok(mut text) = texts.get_mut(children[0]) {
                text.sections[2].value = variable.value.as_string().unwrap();
            }
            if variable.visible {
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        };
    }
}
