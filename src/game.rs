use crate::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Bundle)]
struct AnimalBundle {
    animal: Animal,
    location: Location,
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Animal;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub location: Location,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum TileType {
    Grass,
    Fence,
    Rocks,
    Mud,
    Corral
}

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
pub struct Location {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub offset: Vec3
}

#[derive(Resource)]
pub struct Field {
    pub tiles: Vec<Vec<Entity>>,
    pub objects: Vec<Vec<Option<Entity>>>,
    pub cursor: Entity
}

impl Field {
    pub fn new(commands: &mut Commands, sprites: &Res<Sprites>, width: usize, height: usize) -> Self
    {
        let mut tiles = vec![];
        let mut x = 0;
        while x < width {
            tiles.push(vec![]);
            let mut y = 0;
            while y < height {
                tiles[x].push(commands.spawn(
                    TileBundle {
                        tile: Tile { tile_type: TileType::Grass,
                            location: Location { 
                                x: x,
                                y: y,
                                z: 0,
                                offset: Vec3::splat(0.0)
                            },
                        },
                        sprite: SpriteSheetBundle {
                            texture_atlas: sprites.sprites["Grass"].clone(),
                            sprite: TextureAtlasSprite::new(3),
                            ..default()
                        }
                    }
                ).id());
                y += 1;
            }
            x += 1;
        }
        let objects = vec![vec![None; height]; width];

        let cursor = commands.spawn(
            SpriteSheetBundle {
                texture_atlas: sprites.sprites["Cursor"].clone(),
                sprite: TextureAtlasSprite::new(2),
                ..default()
            }).id();
        let field = Field { tiles, objects, cursor };
        return field;
    }

    pub fn can_get_tile(&self, x: usize, y: usize) -> bool {
        if x < self.tiles.len() && y < self.tiles[x].len() {
            return true;
        }
        return false;
    }

    pub fn get_tile_type(&self, x: usize, y: usize, q_tile: Query<&Tile>) -> Option<TileType> {
        if self.can_get_tile(x, y) {
            if let Ok(tile) = q_tile.get(self.tiles[x][y]) {
                return Some(tile.tile_type);
            }
        }
        return None;
    }

    pub fn set_tile(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, tile_type: TileType, index: usize, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            commands.entity(self.tiles[x][y]).despawn_recursive();
            match tile_type {
                TileType::Grass => {
                    self.tiles[x][y] = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Grass,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                    offset: Vec3::splat(0.0)
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new(index),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Fence => {
                    self.spawn_fence(commands, &sprites, index, x, y);
                }
                TileType::Mud => {
                    self.tiles[x][y] = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Mud,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                    offset: Vec3::splat(0.0)
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Mud"].clone(),
                                sprite: TextureAtlasSprite::new(6),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Rocks => {
                    self.tiles[x][y] = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Rocks,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                    offset: Vec3::splat(0.0)
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Rocks"].clone(),
                                sprite: TextureAtlasSprite::new(6),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Corral => {
                    self.tiles[x][y] = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Corral,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                    offset: Vec3::splat(0.0)
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new(index),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(4+5),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.1 }),
                                ..default()
                            }
                        );
                    }).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(4+5*2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
                _ => {}
            }
        }
    }
    
    fn spawn_fence(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, index: usize, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            self.tiles[x][y] = commands.spawn((
                TileBundle {
                    tile: Tile { 
                        tile_type: TileType::Fence,
                        location: Location { 
                            x: x,
                            y: y,
                            z: 0,
                            offset: Vec3::splat(0.0)
                        },
                    },
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Grass"].clone(),
                        sprite: TextureAtlasSprite::new(index),
                        ..default()
                    }
                }, Fence
                )
            ).with_children(|parent| {
                parent.spawn(
                    SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Fence"].clone(),
                        sprite: TextureAtlasSprite::new(4),
                        transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.1 }),
                        ..default()
                    }
                );
            }).with_children(|parent| {
                parent.spawn(
                    SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Fence"].clone(),
                        sprite: TextureAtlasSprite::new(1),
                        visibility: Visibility::Hidden,
                        transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.2 }),
                        ..default()
                    }
                );
            }).with_children(|parent| {
                parent.spawn(
                    SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Fence"].clone(),
                        sprite: TextureAtlasSprite::new(0),
                        visibility: Visibility::Hidden,
                        transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.2 }),
                        ..default()
                    }
                );
            }).with_children(|parent| {
                parent.spawn(
                    SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Fence"].clone(),
                        sprite: TextureAtlasSprite::new(3),
                        visibility: Visibility::Hidden,
                        transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.2 }),
                        ..default()
                    }
                );
            }).with_children(|parent| {
                parent.spawn(
                    SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Fence"].clone(),
                        sprite: TextureAtlasSprite::new(2),
                        visibility: Visibility::Hidden,
                        transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 0.2 }),
                        ..default()
                    }
                );
            }).id();
        }
    }
}

pub fn setup_level(mut commands: Commands, sprites: Res<Sprites>){
    let mut field = Field::new(&mut commands, &sprites, 14, 8);
    
    let plan = vec![
        vec![0,0,1,1,1,1,1,1,1,1,1,1,1,1],
        vec![0,0,1,0,0,0,0,0,0,0,0,0,0,1],
        vec![0,0,1,0,0,0,0,0,0,0,0,0,0,1],
        vec![0,0,1,0,0,0,0,0,2,2,2,0,0,1],
        vec![0,0,1,0,0,0,0,3,1,1,1,0,0,1],
        vec![0,0,1,0,0,0,3,4,1,5,0,0,0,1],
        vec![1,1,1,0,0,0,0,3,1,0,0,0,0,1],
        vec![1,0,6,0,0,0,0,0,0,0,0,0,0,1]
    ];

    let mut x = 0;
    while x < 14 {
        let mut y = 0;
        while y < 8 {
            match plan[7-y][x] {
                1 => {field.set_tile(&mut commands, &sprites, TileType::Fence, 3, x, y);}
                2 => {field.set_tile(&mut commands, &sprites, TileType::Mud, 0, x, y);}
                3 => {field.set_tile(&mut commands, &sprites, TileType::Rocks, 0, x, y);}
                5 => {field.set_tile(&mut commands, &sprites, TileType::Corral, 0, x, y);}
                _ => {}
            }
            y += 1;
        }
        x += 1;
    }

    commands.spawn(
        AnimalBundle {
            animal: Animal,
            location: Location { 
                x: 2,
                y: 0,
                z: 2,
                offset: Vec3::splat(0.0)
            },
            sprite: SpriteSheetBundle {
                texture_atlas: sprites.sprites["Goat"].clone(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            }
        }
    );

    /*field.set_tile(&mut commands, &sprites, TileType::Fence, 0, 5, 5);
    field.set_tile(&mut commands, &sprites, TileType::Fence, 1, 5, 4);
    field.set_tile(&mut commands, &sprites, TileType::Fence, 2, 4, 5);
    field.set_tile(&mut commands, &sprites, TileType::Fence, 3, 4, 4);*/

    commands.insert_resource(field);
}

pub fn fence_system(field: ResMut<Field>, fences: Query<(&Children, &Tile), With<Fence>>, mut fenceparts: Query<(&TextureAtlasSprite, &mut Visibility), Without<Fence>>){
    for (children, tile) in &fences {
        match tile.tile_type {
            TileType::Fence => {
                for child in children {
                    if let Ok((sprite, mut visibility)) = fenceparts.get_mut(*child) {
                        let locx = tile.location.x;
                        let locy = tile.location.y;
                        *visibility = match sprite.index {
                            0 => {
                                if locy > 0 && field.can_get_tile(locx, locy - 1) && fences.contains(field.tiles[locx][locy - 1]) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            1 => {
                                if field.can_get_tile(locx, locy + 1) && fences.contains(field.tiles[locx][locy + 1]) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            2 => {
                                if locx > 0 && field.can_get_tile(locx - 1, locy) && fences.contains(field.tiles[locx - 1][locy]) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            3 => {
                                if field.can_get_tile(locx + 1, locy) && fences.contains(field.tiles[locx + 1][locy]) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            _ => {Visibility::Visible}
                        };
                    }
                }
            }
            _ => {}
        }
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
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::flex(1.0), GridTrack::auto(), GridTrack::flex(1.0), GridTrack::auto()],
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        " O",
                        text_style.to_owned()
                    ));
                    parent.spawn(TextBundle::from_section(
                        "",
                        text_style.to_owned()
                    ));
                    parent.spawn(TextBundle::from_section(
                        "Level 1: The GOAT",
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

pub fn mouse_controls(
    mut commands: Commands, 
    sprites: Res<Sprites>,
    mut field: ResMut<Field>, 
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_tile: Query<&Tile>,
    mut q_transform: Query<&mut Transform>,
    buttons: Res<Input<MouseButton>>,
    ui_scale: Res<UiScale>,){
    if let Some(position) = q_windows.single().cursor_position() {
        let tile = Vec2{ x: position.x / TILE_SIZE / ui_scale.scale as f32 - ASPECT_RATIO_W / 2.0, y: ASPECT_RATIO_H / 2.0 - position.y / TILE_SIZE / ui_scale.scale as f32};
        let tile_pos_x = (tile.x + TILE_OFFSET_X).round() as usize;
        let tile_pos_y = (tile.y + TILE_OFFSET_Y).round() as usize;
        if field.can_get_tile(tile_pos_x, tile_pos_y) {
            if buttons.just_pressed(MouseButton::Left) {
                match field.get_tile_type(tile_pos_x, tile_pos_y, q_tile) {
                    Some(TileType::Grass) => {field.set_tile(&mut commands, &sprites, TileType::Fence, 3, tile_pos_x, tile_pos_y);}
                    Some(TileType::Fence) => {field.set_tile(&mut commands, &sprites, TileType::Mud, 0, tile_pos_x, tile_pos_y);}
                    Some(TileType::Mud) => {field.set_tile(&mut commands, &sprites, TileType::Rocks, 0, tile_pos_x, tile_pos_y);}
                    Some(TileType::Rocks) => {field.set_tile(&mut commands, &sprites, TileType::Corral, 0, tile_pos_x, tile_pos_y);}
                    Some(TileType::Corral) => {field.set_tile(&mut commands, &sprites, TileType::Grass, 3, tile_pos_x, tile_pos_y);}
                    _ => {}
                }
            }
            if let Ok(mut cursor) = q_transform.get_mut(field.cursor) {
                cursor.scale = Vec3::splat(ui_scale.scale as f32);
                cursor.translation = Vec3{ x: (tile.x.floor() + 0.5) * TILE_SIZE * cursor.scale.x, y: tile.y.round() * TILE_SIZE * cursor.scale.y, z:10.0 };
            }
        }
    }
}