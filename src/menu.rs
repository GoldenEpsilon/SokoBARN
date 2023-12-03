use crate::*;

use bevy::prelude::*;


#[derive(Component)]
#[derive(Default)]
pub struct MenuButton{
    pub button_effect: ButtonEffect,
    pub pickup_object: EntityType,
    pub level: String,
    pub hovering: bool,
    pub hover_time: f32,
    pub editor_mode: Option<bool>
}

#[derive(Component)]
pub struct ButtonDisabled {
    entity: EntityType
}

#[derive(Component)]
pub struct Description;

#[derive(Component)]
pub struct RoundCounter;

#[derive(Resource)]
pub struct MenuData {
    pub button_entities: Vec<Entity>,
}

#[derive(Resource)]
pub struct WorldList {
    pub worlds: Vec<LevelWorld>,
    pub index: usize
}

pub struct LevelWorld {
    pub name: String,
    pub levels: Vec<LevelData>
}

#[derive(Default)]
pub struct LevelData {
    pub name: String,
    pub id: String,
    pub par: usize,
    pub author_par: usize,
    pub record: usize,
    pub unlock_req: usize,
    pub weather: WeatherType,
    pub editor: bool
}

#[derive(Resource)]
pub struct PauseMenuData {
    pub button_entities: Vec<Entity>,
    pub mode: PauseMenuMode
}

#[derive(Resource)]
pub struct SaveRes {
    pub saving: SaveStage,
    pub save: String,
    pub quicksaves: Vec<(String, SimulateRes)>,
    pub editor_mode: Option<bool>
}

#[derive(PartialEq)]
pub enum PauseMenuMode{
    Pause,
    Editor,
    Win,
    Lose
}

#[derive(PartialEq)]
pub enum SaveStage{
    Idle,
    Saving,
    Loading,
    SaveUndo,
    Undo,
}

#[derive(PartialEq)]
#[derive(Default)]
pub enum ButtonEffect{
    #[default] None,
    PrevWorld,
    NextWorld,
    LevelSelect,
    MainMenu,
    Play,
    Quit,
    Settings,
    PickUp,
    Start,
    Save,
    Load,
    Reload,
    Undo,
    Pause,
    UnPause,
}

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, ui_images: Res<UIImages>, mut keyart_q: Query<&mut Visibility, With<KeyArt>>) {

    if let Ok(mut visibility) = keyart_q.get_single_mut() {
        *visibility = Visibility::Visible;
    }

    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    let image = ui_images.sprites["UISign"].to_owned();
    
    let button_entities = vec![
        commands.spawn(NodeBundle {
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
            parent.spawn(ImageBundle {
                image: UiImage::new(ui_images.sprites["UILogo"].to_owned()),
                style: Style {
                    top: Val::Px(-50.0),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
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
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            }, 
            MenuButton{
                button_effect: ButtonEffect::LevelSelect,
                pickup_object: EntityType::None,
                level: "".to_owned(),
                hovering: false, 
                hover_time: 0.0,
                ..default()
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
                pickup_object: EntityType::None,
                level: "".to_owned(),
                hovering: false, 
                hover_time: 0.0,
                ..default()
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

pub fn level_select_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    ui_images: Res<UIImages>, 
    mut menu_data: ResMut<MenuData>,
    sprites: Res<Sprites>,
    world_data: Res<WorldList>) {
    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    let image = ui_images.sprites["UISign"].to_owned();
    
    let world = &world_data.worlds[world_data.index];

    let mut menu = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            justify_items: JustifyItems::Center,
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
            grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::auto()],
            ..default()
        },
        background_color: Color::rgba(0.2, 0.2, 0.25, 0.8).into(),
        ..default()
    }
    );

    menu.with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(160.0),
                height: Val::Px(32.0),
                //border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                grid_column: GridPlacement::span(2),
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
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    world.name.as_str(),
                    text_style.to_owned()
                ));
            });
        });
    });

    menu.with_children(|parent| {
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
                grid_column: GridPlacement::span(2),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }, 
        MenuButton{
            button_effect: ButtonEffect::PrevWorld,
            pickup_object: EntityType::None,
            level: "".to_owned(),
            hovering: false, 
            hover_time: 0.0,
            ..default()
        }))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(image.clone()),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(AtlasImageBundle {
                    texture_atlas: sprites.sprites["Arrow"].to_owned(),
                    texture_atlas_image: UiTextureAtlasImage{index:2,..default()},
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                });
            });
        });
    });

    for level in &world.levels {
        menu.with_children(|parent| {
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
                button_effect: ButtonEffect::Play,
                pickup_object: EntityType::None,
                level: level.id.to_owned(),
                editor_mode: Some(level.editor),
                hovering: false, 
                hover_time: 0.0,
                ..default()
            }))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(image.clone()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        level.name.to_owned(),
                        text_style.to_owned()
                    ));
                });
            });
        });
    }

    menu.with_children(|parent| {
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
                grid_column: GridPlacement::span(2),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }, 
        MenuButton{
            button_effect: ButtonEffect::NextWorld,
            pickup_object: EntityType::None,
            level: "".to_owned(),
            hovering: false, 
            hover_time: 0.0,
            ..default()
        }))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::new(image.clone()),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(AtlasImageBundle {
                    texture_atlas: sprites.sprites["Arrow"].to_owned(),
                    texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                });
            });
        });
    });

    menu_data.button_entities = vec![menu.id()];
}

pub fn pause_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, ui_images: Res<UIImages>, field: Res<Field>, mut pause_menu_data: ResMut<PauseMenuData>) {
    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    if field.editor_mode && pause_menu_data.mode == PauseMenuMode::Pause {
        pause_menu_data.mode = PauseMenuMode::Editor;
    }

    let image = ui_images.sprites["UISign"].to_owned();

    pause_menu_data.button_entities = vec![
        commands.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.25, 0.8).into(),
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
                button_effect: ButtonEffect::MainMenu,
                pickup_object: EntityType::None,
                level: "".to_owned(),
                hovering: false, 
                hover_time: 0.0,
                ..default()
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
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Main Menu",
                        text_style.to_owned()
                    ));
                });
            });
            if pause_menu_data.mode == PauseMenuMode::Lose || pause_menu_data.mode == PauseMenuMode::Pause || pause_menu_data.mode == PauseMenuMode::Editor {
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
                    button_effect: ButtonEffect::Reload,
                    pickup_object: EntityType::None,
                    level: "".to_owned(),
                    hovering: false, 
                    hover_time: 0.0,
                    ..default()
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
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Restart",
                            text_style.to_owned()
                        ));
                    });
                });
            }
            if pause_menu_data.mode == PauseMenuMode::Lose || pause_menu_data.mode == PauseMenuMode::Win {
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
                    button_effect: ButtonEffect::Undo,
                    pickup_object: EntityType::None,
                    level: "".to_owned(),
                    hovering: false, 
                    hover_time: 0.0,
                    ..default()
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
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Undo",
                            text_style.to_owned()
                        ));
                    });
                });
            }
            if pause_menu_data.mode == PauseMenuMode::Editor {
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
                    button_effect: ButtonEffect::Save,
                    pickup_object: EntityType::None,
                    level: "".to_owned(),
                    hovering: false, 
                    hover_time: 0.0,
                    ..default()
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
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Save",
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
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                }, 
                MenuButton{
                    button_effect: ButtonEffect::Load,
                    pickup_object: EntityType::None,
                    level: "".to_owned(),
                    hovering: false, 
                    hover_time: 0.0,
                    ..default()
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
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Load",
                            text_style.to_owned()
                        ));
                    });
                });
            }
            if pause_menu_data.mode == PauseMenuMode::Pause || pause_menu_data.mode == PauseMenuMode::Editor {
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
                    button_effect: ButtonEffect::UnPause,
                    pickup_object: EntityType::None,
                    level: "".to_owned(),
                    hovering: false, 
                    hover_time: 0.0,
                    ..default()
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
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Continue",
                            text_style.to_owned()
                        ));
                    });
                });
            }
        }).id()];
}

pub fn game_ui_setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    sprites: Res<Sprites>, 
    ui_images: Res<UIImages>, 
    mut menu_data: ResMut<MenuData>,
    mut keyart_q: Query<&mut Visibility, With<KeyArt>>) {

    if let Ok(mut visibility) = keyart_q.get_single_mut() {
        *visibility = Visibility::Hidden;
    }

    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };
    let small_text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 8.0,
        ..default()
    };

    let rightpanel = ui_images.sprites["UIRight"].to_owned();
    let bottompanel = ui_images.sprites["UIBottom"].to_owned();
    
    menu_data.button_entities = vec![
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
                    image: UiImage::new(rightpanel.clone()),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::auto(), GridTrack::flex(1.0)],
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE * 2.0),
                            height: Val::Px(TILE_SIZE),
                            grid_column: GridPlacement::span(2),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn((TextBundle::from_section(
                            "Round 0",
                            small_text_style.to_owned()
                        ), RoundCounter));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::ChickenFood,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(AtlasImageBundle {
                            texture_atlas: sprites.sprites["Food"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        });
                        parent.spawn((AtlasImageBundle {
                            texture_atlas: sprites.sprites["Disabled"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        }, ButtonDisabled { entity: EntityType::ChickenFood }));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::HorseFood,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(AtlasImageBundle {
                            texture_atlas: sprites.sprites["Food"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:1,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        });
                        parent.spawn((AtlasImageBundle {
                            texture_atlas: sprites.sprites["Disabled"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        }, ButtonDisabled { entity: EntityType::HorseFood }));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::PigFood,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(AtlasImageBundle {
                            texture_atlas: sprites.sprites["Food"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:2,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        });
                        parent.spawn((AtlasImageBundle {
                            texture_atlas: sprites.sprites["Disabled"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        }, ButtonDisabled { entity: EntityType::PigFood }));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::AllFood,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(AtlasImageBundle {
                            texture_atlas: sprites.sprites["Food"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:3,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        });
                        parent.spawn((AtlasImageBundle {
                            texture_atlas: sprites.sprites["Disabled"].to_owned(),
                            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            visibility: Visibility::Hidden,
                            background_color: Color::WHITE.into(),
                            ..Default::default()
                        }, ButtonDisabled { entity: EntityType::AllFood }));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::PickUp,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                    });
                }).with_children(|parent| {
                    parent.spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            left: Val::Px(2.0),
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::None,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        let mut text = TextBundle::from_section(
                            "A:",
                            small_text_style.to_owned()
                        );
                        text.text.alignment = TextAlignment::Center;
                        parent.spawn((text, Description));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            left: Val::Px(2.0),
                            width: Val::Px(TILE_SIZE),
                            height: Val::Px(TILE_SIZE),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::None,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        let mut text = TextBundle::from_section(
                            "B",
                            small_text_style.to_owned()
                        );
                        text.text.alignment = TextAlignment::Center;
                        parent.spawn((text, Description));
                    });
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            left: Val::Px(2.0),
                            width: Val::Px(TILE_SIZE * 2.0),
                            height: Val::Px(TILE_SIZE * 2.0),
                            grid_column: GridPlacement::span(2),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    }, 
                    MenuButton{
                        button_effect: ButtonEffect::None,
                        pickup_object: EntityType::None,
                        level: "".to_owned(),
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        let mut text = TextBundle::from_section(
                            "Mud: Slippery. Things can't stop here!",
                            small_text_style.to_owned()
                        );
                        text.text.alignment = TextAlignment::Center;
                        parent.spawn((text, Description));
                    });
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
                        image: UiImage::new(bottompanel.clone()),
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
                            button_effect: ButtonEffect::Pause,
                            pickup_object: EntityType::None,
                            level: "".to_owned(),
                            hovering: false, 
                            hover_time: 0.0,
                            ..default()
                        })).with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Menu",
                                text_style.to_owned()
                            ));
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
                            button_effect: ButtonEffect::Undo,
                            pickup_object: EntityType::None,
                            level: "".to_owned(),
                            hovering: false, 
                            hover_time: 0.0,
                            ..default()
                        })).with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Undo",
                                text_style.to_owned()
                            ));
                        });
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
                        parent.spawn(TextBundle::from_section(
                            "Level 1-1",
                            text_style.to_owned()
                        ));
                        parent.spawn(TextBundle::from_section(
                            "",
                            text_style.to_owned()
                        ));
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
                            button_effect: ButtonEffect::Start,
                            pickup_object: EntityType::None,
                            level: "".to_owned(),
                            hovering: false, 
                            hover_time: 0.0,
                            ..default()
                        })).with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "START",
                                text_style.to_owned()
                            ));
                        });
                    });
                });
            });
        }).id()
    ];
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut MenuButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut disabler_q: Query<(&mut Visibility, &ButtonDisabled)>,
    entity_q: Query<&GameEntity>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut saving: ResMut<SaveRes>,
    mut simulating: ResMut<SimulateRes>,
    mut reload_level_select: ResMut<ReloadLevelSelect>,
    mut pause_menu_data: ResMut<PauseMenuData>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut cursor_q: Query<&mut Cursor>,
    mut round_counter_q: Query<&mut Text, With<RoundCounter>>,
    mut world_data: ResMut<WorldList>
) {
    for (mut visibility, disabler) in &mut disabler_q {
        *visibility = Visibility::Hidden;
        for entity in &entity_q {
            if entity.entity_type == disabler.entity {
                *visibility = Visibility::Visible;
                break;
            }
        }
    }
    for (interaction, mut menu_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match menu_button.button_effect {
                    ButtonEffect::MainMenu => {next_state.set(GameState::Menu);}
                    ButtonEffect::Pause => {
                        pause_menu_data.mode = PauseMenuMode::Pause;
                        next_state.set(GameState::Pause);
                    }
                    ButtonEffect::UnPause => {next_state.set(GameState::Gameplay);}
                    ButtonEffect::LevelSelect => {next_state.set(GameState::LevelSelect);}
                    ButtonEffect::Play => {
                        next_state.set(GameState::Gameplay);
                        saving.saving = SaveStage::Loading;
                        saving.save = menu_button.level.to_owned();
                        saving.editor_mode = menu_button.editor_mode;
                    }
                    ButtonEffect::Quit => {app_exit_events.send(bevy::app::AppExit);}
                    ButtonEffect::Start => {
                        if simulating.simulating == false && !simulating.loss && !simulating.win {
                            simulating.simulating = true;
                            simulating.rounds = simulating.rounds + 1;
                            simulating.loss = false;
                            simulating.win = false;
                            if let Ok(mut round_counter) = round_counter_q.get_single_mut() {
                                round_counter.sections[0].value = format!("Round {}", simulating.rounds);
                            }
                        }
                    }
                    ButtonEffect::Save => {
                        saving.saving = SaveStage::Saving;
                        saving.save = "level.skb".to_owned();
                    }
                    ButtonEffect::Load => {
                        saving.saving = SaveStage::Loading;
                        saving.save = "level.skb".to_owned();
                    }
                    ButtonEffect::Reload => {
                        next_state.set(GameState::Gameplay);
                        saving.saving = SaveStage::Loading;
                    }
                    ButtonEffect::Undo => {
                        next_state.set(GameState::Gameplay);
                        simulating.loss = false;
                        simulating.win = false;
                        saving.saving = SaveStage::Undo;
                    }
                    ButtonEffect::PickUp => {
                        if let Ok(mut cursor) = cursor_q.get_single_mut() {
                            let mut can_pick = true;
                            for entity in &entity_q {
                                if entity.entity_type == menu_button.pickup_object {
                                    can_pick = false;
                                    break;
                                }
                            }
                            if can_pick {
                                cursor.holding = menu_button.pickup_object;
                                cursor.drag_drop = true;
                                cursor.starting_pos = cursor.pos;
                            }
                        }
                    }
                    ButtonEffect::Settings => {}
                    ButtonEffect::NextWorld => {
                        if world_data.index < world_data.worlds.len() - 1 {
                            world_data.index += 1;
                            reload_level_select.reloading = true;
                        }
                    }
                    ButtonEffect::PrevWorld => {
                        if world_data.index > 0 {
                            world_data.index -= 1;
                            reload_level_select.reloading = true;
                        }
                    }
                    _ => {}
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
    menu_data: Res<MenuData>,
    mut reload_level_select: ResMut<ReloadLevelSelect>,
) {
    reload_level_select.reloading = false;
    for entity in &menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
}

pub fn pause_menu_cleanup(
    mut commands: Commands,
    pause_menu_data: Res<PauseMenuData>
) {
    for entity in &pause_menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
}

pub fn game_cleanup(
    mut commands: Commands,
    field: Res<Field>,
    menu_data: Res<MenuData>,
    mut simulating: ResMut<SimulateRes>,
    mut q_cursor: Query<&mut Cursor>, 
) {
    simulating.win = false;
    simulating.loss = false;
    simulating.simulating = false;
    simulating.simulation_step = EntityType::None;
    if let Ok(mut cursor) = q_cursor.get_single_mut() {
        cursor.holding = EntityType::None;
    }
    for entity in &menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
    field.despawn_all(&mut commands);
    commands.remove_resource::<Field>();
}