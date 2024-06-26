use crate::*;

use bevy::prelude::*;
use bevy::audio::{Volume, VolumeLevel};


#[derive(Component)]
#[derive(Default)]
pub struct MenuButton{
    pub button_effect: ButtonEffect,
    pub level: Option<LevelData>,
    pub hovering: bool,
    pub hover_time: f32,
}

#[derive(Component)]
pub struct ButtonDisabled {
    entity: EntityType
}

#[derive(Component)]
pub struct Description{
    pub part: usize
}

#[derive(Component)]
pub struct RoundCounter;

#[derive(Component)]
pub struct TutorialButton;

#[derive(Component)]
pub struct CreditsButton;

#[derive(Component)]
pub struct ParText;

#[derive(Resource)]
pub struct MenuData {
    pub button_entities: Vec<Entity>,
    pub menu_offset: usize,
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
#[derive(Clone)]
pub struct LevelData {
    pub name: String,
    pub id: String,
    pub par: usize,
    pub author_par: usize,
    pub record: usize,
    pub unlock_req: usize,
    pub weather: WeatherType,
    pub editor: bool,
    pub song: String
}

#[derive(Resource)]
pub struct PauseMenuData {
    pub button_entities: Vec<Entity>,
    pub mode: PauseMenuMode
}

#[derive(Resource)]
#[derive(Default)]
pub struct SaveRes {
    pub saving: SaveStage,
    pub save: String,
    pub quicksaves: Vec<(String, SimulateRes)>,
    pub editor_mode: Option<bool>,
    pub weather: Option<WeatherType>,
    pub song: Option<String>,
    pub par: usize,
    pub author_par: usize,
}

#[derive(PartialEq)]
pub enum PauseMenuMode{
    Pause,
    Editor,
    Win,
    Lose
}

#[derive(PartialEq)]
#[derive(Default)]
pub enum SaveStage{
    #[default] Idle,
    Saving,
    Loading,
    SaveUndo,
    Undo,
}

#[derive(PartialEq)]
#[derive(Default)]
#[allow(dead_code)]
pub enum ButtonEffect{
    #[default] None,
    PrevWorld,
    NextWorld,
    LevelSelect,
    MainMenu,
    Play,
    Quit,
    Settings,
    PickUp(GameObjectType, bool),
    Paint(GameObjectType),
    Start,
    Save,
    Load,
    Reload,
    Undo,
    Pause,
    UnPause,
    EndTutorial,
    Credits,
    ExitCredits,
    EditorPageLeft,
    EditorPageRight
}

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, ui_images: Res<UIImages>, music: Res<GameMusic>, music_player: Query<Entity, With<MusicPlayer>>, mut keyart_q: Query<&mut Visibility, With<KeyArt>>) {

    for player in &music_player {
        commands.entity(player).despawn();
    }
    commands.spawn((AudioBundle {
        settings: PlaybackSettings{
            mode: PlaybackMode::Loop,
            volume: Volume::Absolute(VolumeLevel::new(0.75)),
            ..default()
        },
        source: music.songs["Song 1"].to_owned(),
        ..default()
    }, MusicPlayer));

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
                level: None,
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
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            }, 
            MenuButton{
                button_effect: ButtonEffect::Credits,
                level: None,
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
                        "Credits",
                        text_style.to_owned()
                    ));
                });
            });
            if !ONLINE_BUILD {
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
                    level: None,
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
            }
        }).id()];
    commands.insert_resource(MenuData { button_entities, menu_offset: 0 });
}

pub fn level_select_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    ui_images: Res<UIImages>, 
    mut menu_data: ResMut<MenuData>,
    medals: Res<Medals>,
    sprites: Res<Sprites>,
    world_data: Res<WorldList>) {
    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };

    let image = ui_images.sprites["UISign"].to_owned();
    
    let world = &world_data.worlds[world_data.index];

    let backid = 
    {
        let mut back = commands.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(160.0),
                height: Val::Px(32.0),
                //border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Percent(1.0),
                top: Val::Percent(8.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }, 
        MenuButton{
            button_effect: ButtonEffect::MainMenu,
            level: None,
            hovering: false, 
            hover_time: 0.0,
            ..default()
        }));
        back.with_children(|parent| {
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
                    texture_atlas_image: UiTextureAtlasImage{index:3,..default()},
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                });
            });
        });
        back.id()
    };

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
            level: None,
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
                level: Some(level.to_owned()),
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
                    parent.spawn(AtlasImageBundle {
                        texture_atlas: sprites.sprites["Medals"].to_owned(),
                        texture_atlas_image: UiTextureAtlasImage{index:medals.medals[&level.id],..default()},
                        ..default()
                    });
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
            level: None,
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

    menu_data.button_entities = vec![backid, menu.id()];
}

pub fn pause_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, ui_images: Res<UIImages>, sprites: Res<Sprites>, field: Res<Field>, simulating: Res<SimulateRes>, mut pause_menu_data: ResMut<PauseMenuData>) {
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
            if pause_menu_data.mode == PauseMenuMode::Lose || pause_menu_data.mode == PauseMenuMode::Win {
                parent.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(
                            if pause_menu_data.mode == PauseMenuMode::Lose {280.0} else {160.0}
                        ),
                        height: Val::Px(
                            if pause_menu_data.mode == PauseMenuMode::Lose {72.0} else {32.0}
                        ),
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
                    button_effect: ButtonEffect::None,
                    level: None,
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
                        if pause_menu_data.mode == PauseMenuMode::Win {
                            parent.spawn(TextBundle::from_section(
                                "You Win!",
                                text_style.to_owned()
                            ));
                            parent.spawn(AtlasImageBundle {
                                texture_atlas: sprites.sprites["Medals"].to_owned(),
                                texture_atlas_image: UiTextureAtlasImage{index: if simulating.rounds <= field.author_par {3} else if simulating.rounds <= field.par {2} else {1},..default()},
                                ..default()
                            });
                        }
                        if pause_menu_data.mode == PauseMenuMode::Lose {
                            parent.spawn(TextBundle::from_section(
                                "Oh No! Your Animals are in trouble!\nTry Again.",
                                text_style.to_owned()
                            ));
                        }
                    });
                });
            }
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
                level: None,
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
                    level: None,
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
                    level: None,
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
            if pause_menu_data.mode == PauseMenuMode::Editor && !ONLINE_BUILD {
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
                    level: None,
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
                    level: None,
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
                    level: None,
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

pub fn get_buttons(field: &Res<Field>) -> Vec<ButtonEffect> {
    return if field.editor_mode {
        vec![
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::Chicken)),
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::Pig)),
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::Horse)),
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::Goat)),
            ButtonEffect::Paint(GameObjectType::Tile(TileType::ChickenPen)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::PigPen)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::GoatPen)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::HorsePen)), 

            ButtonEffect::Paint(GameObjectType::Entity(EntityType::Wagon)),
            ButtonEffect::Paint(GameObjectType::Tile(TileType::Corral)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::Grass)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::Mud)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::Rocks)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::MuddyRocks)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::Fence)), 
            ButtonEffect::Paint(GameObjectType::Tile(TileType::Ditch)), 
            
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::ChickenFood)), 
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::HorseFood)),
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::PigFood)),
            ButtonEffect::Paint(GameObjectType::Entity(EntityType::AllFood)),
        ]
    } else {
        vec![
            ButtonEffect::PickUp(GameObjectType::Entity(EntityType::ChickenFood), true), 
            ButtonEffect::PickUp(GameObjectType::Entity(EntityType::HorseFood), true),
            ButtonEffect::PickUp(GameObjectType::Entity(EntityType::PigFood), true),
            ButtonEffect::PickUp(GameObjectType::Entity(EntityType::AllFood), true),
        ]
    };
}

pub fn game_ui_setup(mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    sprites: Res<Sprites>, 
    ui_images: Res<UIImages>, 
    mut menu_data: ResMut<MenuData>,
    mut tutorial: ResMut<Tutorial>,
    field: Option<Res<Field>>,
    mut reload_game_ui: ResMut<ReloadGameUI>,
    mut keyart_q: Query<&mut Visibility, With<KeyArt>>) {
    *reload_game_ui = ReloadGameUI(false);

    if let Ok(mut visibility) = keyart_q.get_single_mut() {
        *visibility = Visibility::Hidden;
    }

    let text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 20.0,
        ..default()
    };
    let smallish_text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 14.0,
        ..default()
    };
    let small_text_style = TextStyle {
        font: asset_server.load("Fonts/MessyThicc.ttf"),
        font_size: 9.0,
        ..default()
    };

    let rightpanel = ui_images.sprites["UIRight"].to_owned();
    let bottompanel = ui_images.sprites["UIBottom"].to_owned();

    /*if !tutorial.seen {
        tutorial.seen = true;
        commands.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(10),
            background_color: Color::rgba(0.2, 0.2, 0.25, 0.8).into(),
            ..default()
        }, TutorialButton)).with_children(|parent| {
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            }, 
            MenuButton{
                button_effect: ButtonEffect::EndTutorial,
                level: None,
                hovering: false, 
                hover_time: 0.0,
                ..default()
            })).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Welcome to SokoBARN!\n\nThe Farmer's gone missing, and the animals are picking up his job!\n\nMouse over a tile to see what it does! This is your main source of information.\n\nDrag and drop food onto the map. Your goal is to get each animal and cart into their respective pens.\n\nSee if you can reach the Par score on each level, or even match pace with the Developers!",
                    smallish_text_style.to_owned()
                ));
            });
        });
    }*/

    for entity in &menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }

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
                            smallish_text_style.to_owned()
                        ), RoundCounter));
                    });
                    let buttons = if let Some(ref f) = field {get_buttons(f)} else {vec![]};
                    let buttoncount = buttons.len() - menu_data.menu_offset;
                    let buttonlist = &buttons[menu_data.menu_offset..(menu_data.menu_offset + if buttoncount > 9 {8} else {buttoncount})];
                    for button in buttonlist {
                        match *button {
                            ButtonEffect::PickUp(entity_type, limited) => {
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
                                    button_effect: ButtonEffect::PickUp(entity_type, limited),
                                    level: None,
                                    hovering: false, 
                                    hover_time: 0.0,
                                    ..default()
                                })).with_children(|parent| {
                                    parent.spawn(AtlasImageBundle {
                                        texture_atlas: entity_type.icon_atlas(&sprites),
                                        texture_atlas_image: UiTextureAtlasImage{index:entity_type.icon_index(),..default()},
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            position_type: PositionType::Absolute,
                                            ..Default::default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        ..Default::default()
                                    });
                                    if limited {
                                        if let GameObjectType::Entity(entity) = entity_type {
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
                                            }, ButtonDisabled { entity: entity }));
                                        }
                                    }
                                });
                            }
                            ButtonEffect::Paint(tile_type) => {
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
                                    button_effect: ButtonEffect::Paint(tile_type),
                                    level: None,
                                    hovering: false, 
                                    hover_time: 0.0,
                                    ..default()
                                })).with_children(|parent| {
                                    parent.spawn(AtlasImageBundle {
                                        texture_atlas: tile_type.icon_atlas(&sprites),
                                        texture_atlas_image: UiTextureAtlasImage{index:tile_type.icon_index(),..default()},
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            position_type: PositionType::Absolute,
                                            ..Default::default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        ..Default::default()
                                    });
                                });
                            }
                            _ => {
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
                                });
                            }
                        }
                    }
                    if buttoncount > 9 {
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
                            button_effect: ButtonEffect::EditorPageLeft,
                            level: None,
                            hovering: false, 
                            hover_time: 0.0,
                            ..default()
                        })).with_children(|parent| {
                            parent.spawn(AtlasImageBundle {
                                texture_atlas: sprites.sprites["Arrow"].clone(),
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
                        });
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
                            button_effect: ButtonEffect::EditorPageRight,
                            level: None,
                            hovering: false, 
                            hover_time: 0.0,
                            ..default()
                        })).with_children(|parent| {
                            parent.spawn(AtlasImageBundle {
                                texture_atlas: sprites.sprites["Arrow"].clone(),
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
                        });
                    } else {
                        let mut i = buttoncount;
                        while i < 8 {
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
                            });
                            i += 1;
                        }
                        if menu_data.menu_offset > 0 {
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
                                button_effect: ButtonEffect::EditorPageLeft,
                                level: None,
                                hovering: false, 
                                hover_time: 0.0,
                                ..default()
                            })).with_children(|parent| {
                                parent.spawn(AtlasImageBundle {
                                    texture_atlas: sprites.sprites["Arrow"].clone(),
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
                            });
                        }
                        while i < 10 {
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
                            });
                            i += 1;
                        }
                    }
                }).with_children(|parent| {
                    parent.spawn((ButtonBundle {
                        style: Style {
                            left: Val::Px(2.0),
                            width: Val::Px(TILE_SIZE * 2.0),
                            height: Val::Px(TILE_SIZE * 3.0),
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
                        level: None,
                        hovering: false, 
                        hover_time: 0.0,
                        ..default()
                    })).with_children(|parent| {
                        let mut text = TextBundle::from_section(
                            "Mud: Slippery. Things can't stop here!",
                            small_text_style.to_owned()
                        );
                        text.text.alignment = TextAlignment::Center;
                        parent.spawn((text, Description{part:0}));
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
                            level: None,
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
                            level: None,
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
                        parent.spawn(({TextBundle::from_section(
                            "PAR: ".to_owned(),
                            text_style.to_owned()
                        )}, ParText));
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
                            level: None,
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
    mut commands: Commands, 
    mut interaction_query: Query<
        (
            &Interaction,
            &mut MenuButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut disabler_q: Query<(&mut Visibility, &ButtonDisabled)>,
    entity_q: Query<&GameEntity>,
    screencover_q: Query<Entity, Or<(With<CreditsButton>, With<TutorialButton>)>>,
    time: Res<Time>,
    resmuts: (ResMut<NextState<GameState>>, ResMut<SaveRes>, ResMut<SimulateRes>, ResMut<ReloadLevelSelect>, ResMut<MenuData>, ResMut<PauseMenuData>, ResMut<ReloadGameUI>, ResMut<Events<bevy::app::AppExit>>, ResMut<WorldList>),
    fieldopt: Option<Res<Field>>,
    mut cursor_q: Query<&mut Cursor>,
    mut round_counter_q: Query<&mut Text, With<RoundCounter>>,
    asset_server: Res<AssetServer>, 
) {
    let (mut next_state, mut saving, mut simulating, mut reload_level_select, mut menu_data, mut pause_menu_data, mut reload_game_ui, mut app_exit_events, mut world_data) = resmuts;
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
                        if let Some(level) = &menu_button.level {
                            saving.save = level.id.to_owned();
                            saving.editor_mode = Some(level.editor);
                            saving.weather = Some(level.weather);
                            saving.song = Some(level.song.to_owned());
                            saving.par = level.par;
                            saving.author_par = level.author_par;
                        }
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
                        simulating.loss = false;
                        simulating.win = false;
                        saving.saving = SaveStage::Loading;
                    }
                    ButtonEffect::Undo => {
                        next_state.set(GameState::Gameplay);
                        simulating.loss = false;
                        simulating.win = false;
                        saving.saving = SaveStage::Undo;
                    }
                    ButtonEffect::PickUp(pickup_object, limited) => {
                        if let Ok(mut cursor) = cursor_q.get_single_mut() {
                            let mut can_pick = true;
                            if let GameObjectType::Entity(entity_type) = pickup_object {
                                if limited {
                                    for entity in &entity_q {
                                        if entity.entity_type == entity_type {
                                            can_pick = false;
                                            break;
                                        }
                                    }
                                }
                                if can_pick {
                                    cursor.holding = GameObjectType::Entity(entity_type);
                                    cursor.drag_drop = CursorState::Holding;
                                    cursor.starting_pos = cursor.pos;
                                    cursor.painting = false;
                                }
                            }
                        }
                    }
                    ButtonEffect::Paint(paint_object) => {
                        if let Ok(mut cursor) = cursor_q.get_single_mut() {
                            cursor.holding = paint_object;
                            cursor.drag_drop = CursorState::Placing;
                            cursor.starting_pos = cursor.pos;
                            cursor.painting = true;
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
                    ButtonEffect::EndTutorial => {
                        for prompt in &screencover_q {
                            commands.entity(prompt).despawn();
                        }
                    }
                    ButtonEffect::Credits => {
                        if screencover_q.is_empty() {
                            commands.spawn((NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                z_index: ZIndex::Global(10),
                                background_color: Color::rgba(0.2, 0.2, 0.22, 0.98).into(),
                                ..default()
                            }, CreditsButton)).with_children(|parent| {
                                parent.spawn((ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(85.0),
                                        height: Val::Percent(85.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::NONE.into(),
                                    ..default()
                                }, 
                                MenuButton{
                                    button_effect: ButtonEffect::ExitCredits,
                                    level: None,
                                    hovering: false, 
                                    hover_time: 0.0,
                                    ..default()
                                })).with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "  CREDITS\n\n\n\nDankShamwow: Artist, Game/Level Designer, Team Lead\n\nGoldenEpsilon: Programmer, Game Designer\n\nPattieMurr: Music + Sounds, Level Testing\n\nHEHEHE I AM A SUPAHSTAR SAGA: Music, Memes\n\nIMaginatory: Level Testing, Important Feedback",
                                        TextStyle {
                                            font: asset_server.load("Fonts/MessyThicc.ttf"),
                                            font_size: 14.0,
                                            ..default()
                                        }
                                    ));
                                });
                            });
                        }
                    }
                    ButtonEffect::ExitCredits => {
                        for prompt in &screencover_q {
                            commands.entity(prompt).despawn();
                        }
                    }
                    ButtonEffect::EditorPageLeft => {
                        menu_data.menu_offset = std::cmp::max(menu_data.menu_offset, 8) - 8;
                        *reload_game_ui = ReloadGameUI(true);
                    }
                    ButtonEffect::EditorPageRight => {
                        if let Some(ref field) = fieldopt {
                            if menu_data.menu_offset + 8 < get_buttons(field).len() {
                                menu_data.menu_offset += 8;
                                *reload_game_ui = ReloadGameUI(true);
                            }
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
    mut menu_data: ResMut<MenuData>,
    mut reload_level_select: ResMut<ReloadLevelSelect>,
) {
    reload_level_select.reloading = false;
    for entity in &menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
    menu_data.button_entities = vec![];
}

pub fn pause_menu_cleanup(
    mut commands: Commands,
    mut pause_menu_data: ResMut<PauseMenuData>
) {
    for entity in &pause_menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
    pause_menu_data.button_entities = vec![];
}

pub fn game_cleanup(
    mut commands: Commands,
    field: Res<Field>,
    mut menu_data: ResMut<MenuData>,
    mut simulating: ResMut<SimulateRes>,
    mut q_cursor: Query<&mut Cursor>, 
    rain_q: Query<Entity, With<Raindrop>>,
    mut sprite_q: Query<&mut Sprite>,
    weather: Res<Weather>,
) {
    if let Some(overlay_id) = weather.overlay {
        if let Ok(mut overlay) = sprite_q.get_mut(overlay_id) {
            overlay.color = Color::rgba(0.05, 0.05, 0.25, 0.0);
        }
    }
    for raindrop in &rain_q {
        commands.entity(raindrop).despawn();
    }
    simulating.win = false;
    simulating.loss = false;
    simulating.simulating = false;
    simulating.simulation_step = EntityType::None;
    if let Ok(mut cursor) = q_cursor.get_single_mut() {
        cursor.holding = GameObjectType::None;
    }
    for entity in &menu_data.button_entities {
        commands.entity(*entity).despawn_recursive();
    }
    menu_data.menu_offset = 0;
    field.despawn_all(&mut commands);
    commands.remove_resource::<Field>();
}