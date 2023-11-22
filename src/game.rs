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
struct FoodBundle {
    food: Food,
    location: Location,
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct WagonBundle {
    wagon: Wagon,
    location: Location,
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Animal {
    animal_type: EntityType
}

#[derive(Component)]
pub struct Food {
    food_type: EntityType
}

#[derive(Component)]
pub struct Wagon;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
    pub location: Location,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum EntityType {
    Chicken,
    Horse,
    Pig,
    Goat,
    WagonAnimal,
    ChickenFood,
    HorseFood,
    PigFood,
    AllFood,
    WagonFood,
    Wagon,
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
    pub tiles: Vec<Vec<(Entity, Option<Entity>)>>,
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
                tiles[x].push((commands.spawn(
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
                ).id(), None));
                y += 1;
            }
            x += 1;
        }

        let cursor = commands.spawn(
            SpriteSheetBundle {
                texture_atlas: sprites.sprites["Cursor"].clone(),
                sprite: TextureAtlasSprite::new(2),
                ..default()
            }).id();
        let field = Field { tiles, cursor };
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
            if let Ok(tile) = q_tile.get(self.tiles[x][y].0) {
                return Some(tile.tile_type);
            }
        }
        return None;
    }

    pub fn set_tile(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, tile_type: TileType, index: usize, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            commands.entity(self.tiles[x][y].0).despawn_recursive();
            match tile_type {
                TileType::Grass => {
                    self.tiles[x][y].0 = commands.spawn(
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
                    self.tiles[x][y].0 = commands.spawn(
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
                                sprite: TextureAtlasSprite::new(5 + (x % 2) + 4 * (y % 2)),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Rocks => {
                    self.tiles[x][y].0 = commands.spawn(
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
                                sprite: TextureAtlasSprite::new(5 + (x % 2) + 4 * (y % 2)),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Corral => {
                    self.tiles[x][y].0 = commands.spawn(
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

    pub fn add_entity(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, entity_type: EntityType, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            if let Some(old_entity) = self.tiles[x][y].1 {
                commands.entity(old_entity).despawn_recursive();
            }
            match entity_type {
                EntityType::Chicken | EntityType::Horse | EntityType::Pig | EntityType::Goat | EntityType::WagonAnimal => {
                    commands.spawn(
                        AnimalBundle {
                            animal: Animal { animal_type: entity_type },
                            location: Location { 
                                x: 2,
                                y: 0,
                                z: 2,
                                offset: Vec3::splat(0.0)
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites[
                                    match entity_type {
                                        EntityType::Chicken => {"Chicken"}
                                        EntityType::Horse => {"Horse"}
                                        EntityType::Pig => {"Pig"}
                                        EntityType::Goat => {"Goat"}
                                        EntityType::WagonAnimal => {"Wagon"}
                                        _ => {"Chicken"}
                                    }
                                ].clone(),
                                sprite: TextureAtlasSprite::new(0),
                                ..default()
                            }
                        }
                    );
                }
                EntityType::ChickenFood | EntityType::HorseFood | EntityType::PigFood | EntityType::AllFood | EntityType::WagonFood => {
                    commands.spawn(
                        FoodBundle {
                            food: Food { food_type: entity_type },
                            location: Location { 
                                x: 2,
                                y: 0,
                                z: 2,
                                offset: Vec3::splat(0.0)
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Food"].clone(),
                                sprite: TextureAtlasSprite::new(
                                    match entity_type {
                                        EntityType::ChickenFood => {0}
                                        EntityType::HorseFood => {1}
                                        EntityType::PigFood => {2}
                                        EntityType::AllFood => {3}
                                        EntityType::WagonFood => {4}
                                        _ => {3}
                                    }
                                ),
                                ..default()
                            }
                        }
                    );
                }
                EntityType::Wagon => {
                    commands.spawn(
                        WagonBundle {
                            wagon: Wagon,
                            location: Location { 
                                x: 2,
                                y: 0,
                                z: 2,
                                offset: Vec3::splat(0.0)
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Wagon"].clone(),
                                sprite: TextureAtlasSprite::new(0),
                                ..default()
                            }
                        }
                    );
                }
            }
        }
    }
    
    fn spawn_fence(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, index: usize, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            self.tiles[x][y].0 = commands.spawn((
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
                                if locy > 0 && field.can_get_tile(locx, locy - 1) && fences.contains(field.tiles[locx][locy - 1].0) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            1 => {
                                if field.can_get_tile(locx, locy + 1) && fences.contains(field.tiles[locx][locy + 1].0) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            2 => {
                                if locx > 0 && field.can_get_tile(locx - 1, locy) && fences.contains(field.tiles[locx - 1][locy].0) {
                                    Visibility::Visible
                                } else {
                                    Visibility::Hidden
                                }
                            }
                            3 => {
                                if field.can_get_tile(locx + 1, locy) && fences.contains(field.tiles[locx + 1][locy].0) {
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
        if (tile.x + TILE_OFFSET_X).round() >= 0.0 && (tile.y + TILE_OFFSET_Y).round() >= 0.0 && field.can_get_tile(tile_pos_x, tile_pos_y) {
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