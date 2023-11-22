use crate::*;

use bevy::prelude::*;


#[derive(Component)]
pub struct MenuButton{
    pub button_effect: ButtonEffect,
    pub hovering: bool,
    pub hover_time: f32
}

#[derive(Resource)]
pub struct MenuData {
    button_entities: Vec<Entity>,
}

#[derive(Resource)]
pub struct SaveRes {
    pub saving: SaveStage
}

#[derive(PartialEq)]
pub enum SaveStage{
    Idle,
    Saving,
    Loading
}

#[derive(PartialEq)]
pub enum ButtonEffect{
    Start,
    Quit,
    Settings,
    Save,
    Load,
}

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    let image = asset_server.load("UISign.png");
    
    let button_entities = vec![commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
        )
        .with_children(|parent| {
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(160.0),
                    height: Val::Px(32.0),
                    //border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            }, 
            MenuButton{
                button_effect: ButtonEffect::Start,
                hovering: false, 
                hover_time: 0.0
            }))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(image.clone()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        text_style.to_owned()
                    ));
                });
            });
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(160.0),
                    height: Val::Px(32.0),
                    //border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            }, 
            MenuButton{
                button_effect: ButtonEffect::Quit,
                hovering: false, 
                hover_time: 0.0
            }))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(image.clone()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        text_style.to_owned()
                    ));
                });
            });
        }).id()];
    commands.insert_resource(MenuData { button_entities });
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut MenuButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut saving: ResMut<SaveRes>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>
) {
    for (interaction, mut menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match menu_button.button_effect {
                    ButtonEffect::Start => {next_state.set(GameState::Gameplay);}
                    ButtonEffect::Quit => {app_exit_events.send(bevy::app::AppExit);}
                    ButtonEffect::Save => {saving.saving = SaveStage::Saving;}
                    ButtonEffect::Load => {saving.saving = SaveStage::Loading;}
                    ButtonEffect::Settings => {}
                }
            }
            Interaction::Hovered => {
                menu_button.hovering = true;
                menu_button.hover_time = time.elapsed_seconds();
            }
            Interaction::None => {
                menu_button.hovering = false;
            }
        }
    }
}
pub fn button_update_system(
    buttons: Query<
        (
            &MenuButton,
            &Children
        ),
        With<Button>,
    >,
    mut style_query: Query<&mut Style>,
    time: Res<Time>,){
    for (menu_button, children) in &buttons {
        for child in children {
            if let Ok(mut style) = style_query.get_mut(*child){
                if menu_button.hovering {
                    style.top = Val::Px(f32::sin((time.elapsed_seconds() - menu_button.hover_time) * 5.0) * 4.0);
                } else {
                    style.top = style.top / 1.2;
                }
            }
        }
    }
}

pub fn menu_cleanup(
    mut commands: Commands,
    menu_data: Res<MenuData>
) {
    for entity in &menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
}

pub fn game_ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    let image = asset_server.load("UISign.png");
    
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
    )
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(TILE_SIZE*2.0),
                height: Val::Px(TILE_SIZE*ASPECT_RATIO_H),
                left: Val::Px(TILE_SIZE*ASPECT_RATIO_W/2.0 - TILE_SIZE*1.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(image.clone()),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            });
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(TILE_SIZE*(ASPECT_RATIO_W - 2.0)),
                    height: Val::Px(TILE_SIZE*1.0),
                    top: Val::Px(TILE_SIZE*ASPECT_RATIO_H - TILE_SIZE*1.0),
                    left: Val::Px(-TILE_SIZE*(ASPECT_RATIO_W - 2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(image.clone()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::flex(1.0), GridTrack::auto(), GridTrack::flex(1.0), GridTrack::auto()],
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(96.0),
                            height: Val::Px(32.0),
                            //border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::Save,
                        hovering: false, 
                        hover_time: 0.0
                    })).with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: UiImage::new(image.clone()),
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "SAVE",
                                text_style.to_owned()
                            ));
                        });
                    });
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(96.0),
                            height: Val::Px(32.0),
                            //border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::Load,
                        hovering: false, 
                        hover_time: 0.0
                    })).with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: UiImage::new(image.clone()),
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "LOAD",
                                text_style.to_owned()
                            ));
                        });
                    });
                    parent.spawn(TextBundle::from_section(
                        "",
                        text_style.to_owned()
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Level 1",
                        text_style.to_owned()
                    ));
                    parent.spawn(TextBundle::from_section(
                        "",
                        text_style.to_owned()
                    ));
                    parent.spawn(TextBundle::from_section(
                        "START ",
                        text_style.to_owned()
                    ));
                });
            });
        });
    });
}