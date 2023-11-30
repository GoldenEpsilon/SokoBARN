use crate::*;

use bevy::reflect::TypeUuid;
use bevy::{prelude::*, reflect::TypePath};
use bevy::window::PrimaryWindow;

use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[derive(TypePath)]
#[derive(TypeUuid)]
#[uuid = "71402ca5-adec-436a-ba16-6980791e7c7d"]
pub struct SaveFile {
    version: usize,
    width: usize,
    height: usize,
    //Tile, Buttons, Food, Animals
    tiles: Vec<(Option<Tile>, Option<GameEntity>, Option<GameEntity>, Option<GameEntity>)>,
}

#[derive(Resource)]
#[derive(Default)]
pub struct SimulateRes {
    pub simulating: bool,
    pub rounds: usize
}

#[derive(Bundle)]
struct AnimalBundle {
    entity: GameEntity,
    animal: Animal,
    sprite: SpriteSheetBundle,
    pub animation_timer: AnimationTimer
}

#[derive(Bundle)]
struct FoodBundle {
    entity: GameEntity,
    food: Food,
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct WagonBundle {
    entity: GameEntity,
    wagon: Wagon,
    sprite: SpriteSheetBundle,
    pub animation_timer: AnimationTimer
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    sprite: SpriteSheetBundle,
}

#[derive(Component)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone, Copy)]
pub struct GameEntity {
    pub entity_type: EntityType,
    pub location: Location,
    pub target_location: Location,
    pub offset: Vec2,
    pub state: EntityState,
    pub last_direction: MoveDirection
}

#[derive(Component)]
pub struct Animal;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Wagon;

#[derive(Component)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub location: Location,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayModeTick(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize, Debug)]
pub enum EntityState {
    Idle,
    Walking,
    Sliding,
    Eating,
    Celebrating,
    Special,
    Failure,
}

#[derive(PartialEq, Eq)]
#[derive(Clone, Copy)]
#[derive(PartialOrd, Ord)]
#[derive(Serialize, Deserialize, Debug)]
pub enum EntityType {
    Chicken,
    Pig,
    Horse,
    Goat,
    Wagon,
    ChickenFood,
    HorseFood,
    PigFood,
    AllFood,
    WagonFood,
    WagonAnimal,
    None,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize, Debug)]
pub enum TileType {
    Grass,
    Fence,
    Rocks,
    Mud,
    MuddyRocks,
    Ditch,
    Corral,
    ChickenPen,
    PigPen,
    GoatPen,
    HorsePen,
}

#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MoveDirection {
    None,
    Left,
    Right,
    Up,
    Down
}

#[derive(Component)]
pub struct Fence;

#[derive(Component)]
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone, Copy)]
pub struct Location {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Resource)]
pub struct Field {
    //Tile, Buttons, Food, Animals
    tiles: Vec<Vec<(Entity, Option<Entity>, Option<Entity>, Option<Entity>, Option<Entity>)>>,
    pub cursor: Entity,
    pub simulate_timer: PlayModeTick,
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
                            },
                        },
                        sprite: SpriteSheetBundle {
                            texture_atlas: sprites.sprites["Grass"].clone(),
                            sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                            ..default()
                        }
                    }
                ).id(), None, None, None, None));
                y += 1;
            }
            x += 1;
        }

        let cursor = commands.spawn(
            SpriteSheetBundle {
                texture_atlas: sprites.sprites["Cursor"].clone(),
                sprite: TextureAtlasSprite::new(4),
                ..default()
            }).id();
        let field = Field { tiles, cursor, simulate_timer: PlayModeTick(Timer::from_seconds(0.6, TimerMode::Repeating)) };
        return field;
    }

    pub fn despawn_all(&self, commands: &mut Commands){
        commands.entity(self.cursor).despawn_recursive();
        for column in &self.tiles {
            for (tile, layer0, layer1, layer2, layer3) in column {
                commands.entity(*tile).despawn_recursive();
                if let Some(entity) = layer0 {
                    commands.entity(*entity).despawn_recursive();
                }
                if let Some(entity) = layer1 {
                    commands.entity(*entity).despawn_recursive();
                }
                if let Some(entity) = layer2 {
                    commands.entity(*entity).despawn_recursive();
                }
                if let Some(entity) = layer3 {
                    commands.entity(*entity).despawn_recursive();
                }
            }
        }
    }

    pub fn can_get_tile(&self, x: usize, y: usize) -> bool {
        if x < self.tiles.len() && y < self.tiles[x].len() {
            return true;
        }
        return false;
    }

    pub fn get_tile_type(&self, x: usize, y: usize, q_tile: &Query<&Tile>) -> Option<TileType> {
        if self.can_get_tile(x, y) {
            if let Ok(tile) = q_tile.get(self.tiles[x][y].0) {
                return Some(tile.tile_type);
            }
        }
        return None;
    }

    pub fn get_entity_type(&self, x: usize, y: usize, q_entity: Query<&GameEntity>) -> Option<EntityType> {
        if self.can_get_tile(x, y) {
            if let Some(entity_object) = self.tiles[x][y].3 {
                if let Ok(entity) = q_entity.get(entity_object) {
                    return Some(entity.entity_type);
                }
            }else if let Some(entity_object) = self.tiles[x][y].2 {
                if let Ok(entity) = q_entity.get(entity_object) {
                    return Some(entity.entity_type);
                }
            }
        }
        return None;
    }

    pub fn set_tile(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, tile_type: TileType, x: usize, y: usize){
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
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Fence => {
                    self.spawn_fence(commands, &sprites, x, y);
                }
                TileType::Mud => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Mud,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 2,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Mud"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
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
                                    z: 5,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Rocks"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::MuddyRocks => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::MuddyRocks,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 4,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["MuddyRocks"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).id()
                }
                TileType::Ditch => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Ditch,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Ditch"].clone(),
                                sprite: TextureAtlasSprite::new(15),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 2.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
                TileType::Corral => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::Corral,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(4),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(4+5),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 2.791 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(4+5*2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 3.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
                TileType::ChickenPen => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::ChickenPen,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(0),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(0+5),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 2.791 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(0+5*2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 3.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
                TileType::HorsePen => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::HorsePen,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(1),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(1+5),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 2.791 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(1+5*2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 3.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
                TileType::PigPen => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::PigPen,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(2+5),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 2.791 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(2+5*2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 3.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
                TileType::GoatPen => {
                    self.tiles[x][y].0 = commands.spawn(
                        TileBundle {
                            tile: Tile { tile_type: TileType::GoatPen,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 0,
                                },
                            },
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Grass"].clone(),
                                sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
                                ..default()
                            }
                        }
                    ).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(3),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 1.0 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(3+5),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 2.791 }),
                                ..default()
                            }
                        );
                        parent.spawn(
                            SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Pens"].clone(),
                                sprite: TextureAtlasSprite::new(3+5*2),
                                transform: Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 3.1 }),
                                ..default()
                            }
                        );
                    }).id()
                }
            }
        }
    }

    pub fn set_entity(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, entity_type: EntityType, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            if let Some(old_entity) = self.tiles[x][y].2 {
                commands.entity(old_entity).despawn_recursive();
                self.tiles[x][y].2 = None;
            }
            if let Some(old_entity) = self.tiles[x][y].3 {
                commands.entity(old_entity).despawn_recursive();
                self.tiles[x][y].3 = None;
            }
            match entity_type {
                EntityType::Chicken | EntityType::Horse | EntityType::Pig | EntityType::Goat | EntityType::WagonAnimal => {
                    self.tiles[x][y].3 = Some(commands.spawn(
                        AnimalBundle {
                            entity: GameEntity { 
                                entity_type: entity_type,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 7,
                                },
                                target_location: Location {x,y,z:0},
                                offset: Vec2::splat(0.0),
                                state: EntityState::Idle,
                                last_direction: MoveDirection::None,
                            },
                            animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                            animal: Animal,
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
                    ).id());
                }
                EntityType::ChickenFood | EntityType::HorseFood | EntityType::PigFood | EntityType::AllFood | EntityType::WagonFood => {
                    self.tiles[x][y].2 = Some(commands.spawn(
                        FoodBundle {
                            entity: GameEntity { 
                                entity_type: entity_type,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 7,
                                },
                                target_location: Location {x,y,z:0},
                                offset: Vec2::splat(0.0),
                                state: EntityState::Idle,
                                last_direction: MoveDirection::None,
                            },
                            food: Food,
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
                    ).id());
                }
                EntityType::Wagon => {
                    self.tiles[x][y].3 = Some(commands.spawn(
                        WagonBundle {
                            entity: GameEntity { 
                                entity_type: entity_type,
                                location: Location { 
                                    x: x,
                                    y: y,
                                    z: 7,
                                },
                                target_location: Location {x,y,z:0},
                                offset: Vec2::splat(0.0),
                                state: EntityState::Idle,
                                last_direction: MoveDirection::None,
                            },
                            animation_timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
                            wagon: Wagon,
                            sprite: SpriteSheetBundle {
                                texture_atlas: sprites.sprites["Wagon"].clone(),
                                sprite: TextureAtlasSprite::new(0),
                                ..default()
                            }
                        }
                    ).id());
                }
                _ => {
                }
            }
        }
    }
    
    fn spawn_fence(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, x: usize, y: usize){
        if self.can_get_tile(x, y) {
            self.tiles[x][y].0 = commands.spawn((
                TileBundle {
                    tile: Tile { 
                        tile_type: TileType::Fence,
                        location: Location { 
                            x: x,
                            y: y,
                            z: 0,
                        },
                    },
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Grass"].clone(),
                        sprite: TextureAtlasSprite::new((x % 2) + 2 * (y % 2)),
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

    pub fn can_see_food(&mut self, animal: GameEntity, entity_q: &Query<&GameEntity>, tile_q: &Query<&Tile>,) -> Location {
        let animalx = animal.location.x;
        let animaly = animal.location.y;
        let mut x = animalx;
        let mut y = animaly;
        let mut best = (Location{x,y,z:0}, 999);
        x = animalx + 1;
        y = animaly;
        while self.can_get_tile(x, y){
            if let Ok(tile) = tile_q.get(self.tiles[x][y].0) {
                if tile.tile_type == TileType::Fence {
                    break;
                }
            }
            if self.tiles[x][y].3.is_some() {
                break;
            }
            if self.likes_food_on_tile(animal, &entity_q, x, y) {
                if let Some(entity_id) = self.tiles[x][y].2 {
                    if let Ok(entity) = entity_q.get(entity_id) {
                        if best.1 > x - animalx {best = (entity.location, x - animalx);}
                        break;
                    }
                }
            }
            x = x + 1;
        }
        x = animalx;
        y = animaly + 1;
        while self.can_get_tile(x, y){
            if let Ok(tile) = tile_q.get(self.tiles[x][y].0) {
                if tile.tile_type == TileType::Fence {
                    break;
                }
            }
            if self.tiles[x][y].3.is_some() {
                break;
            }
            if self.likes_food_on_tile(animal, &entity_q, x, y) {
                if let Some(entity_id) = self.tiles[x][y].2 {
                    if let Ok(entity) = entity_q.get(entity_id) {
                        if best.1 > y - animaly {best = (entity.location, y - animaly);}
                        break;
                    }
                }
            }
            y = y + 1;
        }
        if animalx > 0 {x = animalx - 1;}
        y = animaly;
        while self.can_get_tile(x, y){
            if let Ok(tile) = tile_q.get(self.tiles[x][y].0) {
                if tile.tile_type == TileType::Fence {
                    break;
                }
            }
            if self.tiles[x][y].3.is_some() {
                break;
            }
            if self.likes_food_on_tile(animal, &entity_q, x, y) {
                if let Some(entity_id) = self.tiles[x][y].2 {
                    if let Ok(entity) = entity_q.get(entity_id) {
                        if best.1 > animalx - x {best = (entity.location, animalx - x);}
                        break;
                    }
                }
            }
            if x == 0 {break;}
            x = x - 1;
        }
        x = animalx;
        if animaly > 0 {y = animaly - 1;}
        while self.can_get_tile(x, y){
            if let Ok(tile) = tile_q.get(self.tiles[x][y].0) {
                if tile.tile_type == TileType::Fence {
                    break;
                }
            }
            if self.tiles[x][y].3.is_some() {
                break;
            }
            if self.likes_food_on_tile(animal, &entity_q, x, y) {
                if let Some(entity_id) = self.tiles[x][y].2 {
                    if let Ok(entity) = entity_q.get(entity_id) {
                        if best.1 > animaly - y {best = (entity.location, animaly - y);}
                        break;
                    }
                }
            }
            if y == 0 {break;}
            y = y - 1;
        }
        return best.0;
    }

    pub fn likes_food_on_tile(&mut self, animal: GameEntity, entity_q: &Query<&GameEntity>, x: usize, y: usize) -> bool {
        if self.can_get_tile(x, y) {
            if let Some(entity_id) = self.tiles[x][y].2 {
                if let Ok(entity) = entity_q.get(entity_id) {
                    if entity.entity_type == EntityType::AllFood || entity.entity_type == match animal.entity_type {
                        EntityType::Chicken => { EntityType::ChickenFood }
                        EntityType::Pig => { EntityType::PigFood }
                        EntityType::Horse => { EntityType::HorseFood }
                        EntityType::Goat => { entity.entity_type }
                        EntityType::WagonAnimal => { EntityType::WagonFood }
                        _ => { EntityType::AllFood }
                    } {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn get_entities(&mut self,
        entity_q: &Query<&GameEntity>, 
    ) -> Vec<GameEntity>{
        let mut ret_val = vec![];
        for column in &self.tiles {
            for tile in column {
                if let Some(entity_id) = tile.3 {
                    if let Ok(entity) = entity_q.get(entity_id) {
                        ret_val.push(entity.to_owned());
                    }
                }
            }
        }
        ret_val.sort_by_key(|e| e.entity_type);
        return ret_val;
    }

    pub fn slide_entity(&mut self,
        commands: &mut Commands, 
        entity_q: &mut Query<&mut GameEntity>, 
        tile_q: &Query<&Tile>,
        entity: GameEntity,
        slide_direction: MoveDirection) -> bool{

        let startx = entity.location.x;
        let starty = entity.location.y;

        let (xoffset, yoffset) = match slide_direction {
            MoveDirection::Left => {(-1, 0)}
            MoveDirection::Right => {(1, 0)}
            MoveDirection::Down => {(0, -1)}
            MoveDirection::Up => {(0, 1)}
            _ => {(0, 0)}
        };

        if (startx as isize) < -xoffset || (starty as isize) < -yoffset {
            //SLID OUT OF BOUNDS
            return false;
        }
        let x: usize = ((startx as isize) + xoffset) as usize;
        let y: usize = ((starty as isize) + yoffset) as usize;
        return self.move_entity(commands, entity_q, tile_q, entity, Location{x, y, z:0})
    }

    pub fn move_entity(&mut self,
        commands: &mut Commands, 
        entity_q: &mut Query<&mut GameEntity>, 
        tile_q: &Query<&Tile>,
        entity: GameEntity, 
        target_location: Location) -> bool{

        let startx = entity.location.x;
        let starty = entity.location.y;

        let mut target = (target_location.x as isize, target_location.y as isize);
        if (target.0 - (startx as isize)).abs() > (target.1 - (starty as isize)).abs() {target.1 = starty as isize;} else {target.0 = startx as isize;}

        let (xoffset, yoffset): (isize, isize) = ((target.0 - (startx as isize)).signum(), (target.1 - (starty as isize)).signum());

        let move_direction = 
            if xoffset < 0 {
                MoveDirection::Left
            }else if xoffset > 0 {
                MoveDirection::Right
            }else if yoffset < 0 {
                MoveDirection::Down
            }else if yoffset > 0 {
                MoveDirection::Up
            }else{
                MoveDirection::None
            };
        
            println!("{:?}", move_direction);

        if move_direction == MoveDirection::None || 
            (entity.state != EntityState::Sliding && !self.likes_food_on_tile(entity, &entity_q.to_readonly(), target_location.x, target_location.y)) {
            if let Some(entity_id) = self.tiles[startx][starty].3 {
                if let Ok(mut moving_entity) = entity_q.get_mut(entity_id) {
                    moving_entity.state = EntityState::Idle;
                    moving_entity.target_location = moving_entity.location;
                }
            }
            return true;
        }

        if (startx as isize) < -xoffset || (starty as isize) < -yoffset {
            if let Some(entity_id) = self.tiles[startx][starty].3 {
                if let Ok(mut moving_entity) = entity_q.get_mut(entity_id) {
                    moving_entity.state = EntityState::Idle;
                }
            }
            return true;
        }
        let x: usize = ((startx as isize) + xoffset) as usize;
        let y: usize = ((starty as isize) + yoffset) as usize;

        let tile_in_front = (x as isize) >= -xoffset && (y as isize) >= -yoffset;
        let frontx: usize = if tile_in_front {((x as isize) + xoffset) as usize} else {0};
        let fronty: usize = if tile_in_front {((y as isize) + yoffset) as usize} else {0};

        let tile_in_back = (startx as isize) >= xoffset && (starty as isize) >= yoffset;
        let backx: usize = if tile_in_back {((startx as isize) - xoffset) as usize} else {0};
        let backy: usize = if tile_in_back {((starty as isize) - yoffset) as usize} else {0};

        //check entity id to make sure it matches up with entity
        if self.can_get_tile(x, y) && self.can_get_tile(startx, starty) {
            if let Some(entity_id) = self.tiles[startx][starty].3 {
                if let Some(target_entity_id) = self.tiles[x][y].3 {
                    if let Ok([mut entity, mut target_entity]) = entity_q.get_many_mut([entity_id, target_entity_id]) {
                        //check for other animals in the way
                        match target_entity.entity_type {
                            EntityType::None => {

                            }
                            _ => {
                                if entity.entity_type == EntityType::Goat {
                                    let tile_slam_target = (frontx as isize) > -xoffset || (fronty as isize) > -yoffset;
                                    if tile_slam_target && 
                                        self.can_get_tile(((frontx as isize) + xoffset) as usize, ((fronty as isize) + yoffset) as usize) {
                                        let tile_slam_target_x = ((frontx as isize) + xoffset) as usize;
                                        let tile_slam_target_y = ((fronty as isize) + yoffset) as usize;
                                        if self.tiles[tile_slam_target_x][tile_slam_target_y].3.is_some() {
                                            println!("SLAM FAIL");
                                            //FAIL
                                            return false;
                                        }else{
                                            //SLAM
                                            println!("SLAM");
                                            target_entity.last_direction = move_direction;
                                            target_entity.location.x = tile_slam_target_x;
                                            target_entity.location.y = tile_slam_target_y;
                                            target_entity.target_location = target_entity.location;
                                            target_entity.state = EntityState::Idle;
                                            if !(startx == x && starty == y) {
                                                self.tiles[tile_slam_target_x][tile_slam_target_y].3 = self.tiles[frontx][fronty].3.to_owned();
                                                self.tiles[frontx][fronty].3 = None;
                                            }
                                        }
                                    }
                                }
                                entity.state = EntityState::Idle;
                                return true;
                            }
                        }
                    }
                }
                if tile_in_front && self.can_get_tile(frontx, fronty) {
                    if let Some(slam_entity_id) = self.tiles[frontx][fronty].3 {
                        println!("CHECKING FOR SLAM");
                        if let Ok([entity, mut slam_entity]) = entity_q.get_many_mut([entity_id, slam_entity_id]) {
                            if entity.entity_type == EntityType::Goat {
                                println!("TRYING TO SLAM");
                                let tile_slam_target = (frontx as isize) > -xoffset*2 || (fronty as isize) > -yoffset*2;
                                if tile_slam_target && 
                                self.can_get_tile(((frontx as isize) + xoffset*2) as usize, ((fronty as isize) + yoffset*2) as usize) {
                                    let tile_slam_target_x = ((frontx as isize) + xoffset*2) as usize;
                                    let tile_slam_target_y = ((fronty as isize) + yoffset*2) as usize;
                                    if self.tiles[tile_slam_target_x][tile_slam_target_y].3.is_some() {
                                        println!("SLAM FAIL");
                                        //FAIL
                                        return false;
                                    }else{
                                        //SLAM
                                        println!("SLAM");
                                        slam_entity.last_direction = move_direction;
                                        slam_entity.location.x = tile_slam_target_x;
                                        slam_entity.location.y = tile_slam_target_y;
                                        slam_entity.target_location = slam_entity.location;
                                        if !(startx == x && starty == y) {
                                            self.tiles[tile_slam_target_x][tile_slam_target_y].3 = self.tiles[frontx][fronty].3.to_owned();
                                            self.tiles[frontx][fronty].3 = None;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(food_entity_id) = self.tiles[x][y].2 {
                    if let Ok(food_entity) = entity_q.get(food_entity_id) {
                        if food_entity.entity_type == EntityType::AllFood {
                            commands.entity(food_entity_id).despawn_recursive();
                            self.tiles[x][y].2 = None;
                        } else {
                            match entity.entity_type {
                                EntityType::Chicken => {
                                    if food_entity.entity_type == EntityType::ChickenFood {
                                        commands.entity(food_entity_id).despawn_recursive();
                                        self.tiles[x][y].2 = None;
                                    }
                                }
                                EntityType::Pig => {
                                    if food_entity.entity_type == EntityType::PigFood {
                                        commands.entity(food_entity_id).despawn_recursive();
                                        self.tiles[x][y].2 = None;
                                    }
                                }
                                EntityType::Horse => {
                                    if food_entity.entity_type == EntityType::HorseFood {
                                        commands.entity(food_entity_id).despawn_recursive();
                                        self.tiles[x][y].2 = None;
                                    }
                                }
                                EntityType::Goat => {
                                    commands.entity(food_entity_id).despawn_recursive();
                                    self.tiles[x][y].2 = None;
                                }
                                EntityType::Wagon => {
                                    if food_entity.entity_type == EntityType::WagonFood {
                                        commands.entity(food_entity_id).despawn_recursive();
                                        self.tiles[x][y].2 = None;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                if let Ok(mut moving_entity) = entity_q.get_mut(entity_id) {
                    if let Ok(tile) = tile_q.get(self.tiles[x][y].0) {
                        //TODO: finish terrain code here
                        match tile.tile_type {
                            TileType::Fence => {
                                moving_entity.state = EntityState::Idle;
                                return true;
                            }
                            TileType::Mud => {
                                //set state as muddy
                                if moving_entity.entity_type != EntityType::Pig {
                                    moving_entity.state = EntityState::Sliding;
                                }
                            }
                            TileType::MuddyRocks => {
                                //set state as muddy
                                if moving_entity.entity_type != EntityType::Pig {
                                    moving_entity.state = EntityState::Sliding;
                                }
                            }
                            TileType::Ditch => {
                                if moving_entity.entity_type == EntityType::Chicken && moving_entity.state != EntityState::Special {
                                    moving_entity.state = EntityState::Special;
                                }else{
                                    moving_entity.state = EntityState::Failure;
                                    println!("DITCH FAIL");
                                    return false; //FAIL
                                }
                            }
                            _ => {
                                moving_entity.state = EntityState::Walking;
                            }
                        }
                        //self.set_entity(commands, &sprites, entity.entity_type, x, y);
                        //self.set_entity(commands, &sprites, EntityType::None, entityx, entityy);
                    }
                    if entity.state != EntityState::Sliding {
                        moving_entity.target_location = target_location;
                        if moving_entity.target_location.x == moving_entity.location.x && moving_entity.target_location.y == moving_entity.location.y {
                            moving_entity.state = EntityState::Idle;
                        }
                    }

                    moving_entity.last_direction = move_direction;
                    moving_entity.location.x = x;
                    moving_entity.location.y = y;
                    if !(startx == x && starty == y) {
                        self.tiles[x][y].3 = self.tiles[startx][starty].3.to_owned();
                        self.tiles[startx][starty].3 = None;
                    } else {
                        moving_entity.state = EntityState::Idle;
                    }
                }
                if tile_in_back && self.can_get_tile(backx, backy) {
                    if let Some(pull_entity_id) = self.tiles[backx][backy].3 {
                        if let Ok([entity, mut pull_entity]) = entity_q.get_many_mut([entity_id, pull_entity_id]) {
                            if entity.entity_type == EntityType::Horse && pull_entity.entity_type == EntityType::Wagon {
                                println!("TRYING TO PULL");
                                if self.tiles[startx][starty].3.is_none() {
                                    pull_entity.last_direction = move_direction;
                                    pull_entity.location.x = startx;
                                    pull_entity.location.y = starty;
                                    self.tiles[startx][starty].3 = self.tiles[backx][backy].3.to_owned();
                                    self.tiles[backx][backy].3 = None;
                                } else {
                                    println!("PULL FAILED");
                                }
                            }
                        }
                    }
                }
            }
        }
        return true;
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
                1 => {field.set_tile(&mut commands, &sprites, TileType::Fence, x, y);}
                2 => {field.set_tile(&mut commands, &sprites, TileType::Mud, x, y);}
                3 => {field.set_tile(&mut commands, &sprites, TileType::Rocks, x, y);}
                5 => {field.set_tile(&mut commands, &sprites, TileType::Corral, x, y);}
                _ => {}
            }
            y += 1;
        }
        x += 1;
    }

    field.set_entity(&mut commands, &sprites, EntityType::Goat, 2, 0);

    field.set_entity(&mut commands, &sprites, EntityType::AllFood, 5, 0);

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
    q_entity: Query<&GameEntity>,
    mut q_cursor: Query<&mut Cursor>, 
    mut q_transform: Query<&mut Transform>,
    mut q_desc: Query<&mut Text, With<Description>>,
    buttons: Res<Input<MouseButton>>,
    ui_scale: Res<UiScale>,){
    if let Ok(window) = q_windows.get_single() {
        if let Some(position) = window.cursor_position() {
            let tile = Vec2{ x: (position.x - window.width()/2.0) / TILE_SIZE / ui_scale.scale as f32, y: (window.height()/2.0 - position.y) / TILE_SIZE / ui_scale.scale as f32};
            let tile_pos_x = (tile.x + TILE_OFFSET_X).round() as usize;
            let tile_pos_y = (tile.y + TILE_OFFSET_Y).round() as usize;
            if let Ok(mut cursor) = q_cursor.get_single_mut() {
                if cursor.holding != EntityType::None 
                    && (buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right)) != cursor.drag_drop 
                    && (Vec2::distance(cursor.pos, cursor.starting_pos) > CURSOR_MIN_MOVE_DIST || buttons.just_pressed(MouseButton::Left)) {
                    match field.get_tile_type(tile_pos_x, tile_pos_y, &q_tile) {
                        Some(TileType::Fence) | Some(TileType::Ditch) => {}
                        _ => {
                            field.set_entity(&mut commands, &sprites, cursor.holding, tile_pos_x, tile_pos_y);
                            cursor.holding = EntityType::None;
                        }
                    }
                }
                if buttons.just_released(MouseButton::Left) && Vec2::distance(cursor.pos, cursor.starting_pos) < CURSOR_MIN_MOVE_DIST {
                    cursor.drag_drop = false;
                }
                if cursor.holding == EntityType::None {
                    if buttons.just_pressed(MouseButton::Left) {
                        let food = field.get_entity_type(tile_pos_x, tile_pos_y, q_entity);
                        match food {
                            Some(EntityType::ChickenFood) | Some(EntityType::HorseFood) | Some(EntityType::PigFood) | Some(EntityType::AllFood) | Some(EntityType::WagonFood) => {
                                cursor.holding = food.unwrap();
                                if let Some(old_entity) = field.tiles[tile_pos_x][tile_pos_y].2 {
                                    commands.entity(old_entity).despawn_recursive();
                                    field.tiles[tile_pos_x][tile_pos_y].2 = None;
                                }
                                cursor.starting_pos = cursor.pos;
                            }
                            _ => {}
                        }
                        cursor.drag_drop = true;
                    }
                }
            }
            if field.can_get_tile(tile_pos_x, tile_pos_y) {
                if let Ok(mut desc) = q_desc.get_single_mut() {
                    desc.sections[0].value = 
                    match field.get_tile_type(tile_pos_x, tile_pos_y, &q_tile) {
                        Some(TileType::Fence) => {"Fence: Impassible. Keeps everything in, no matter what!"}
                        Some(TileType::Mud) => {"Mud: Slippery. Things can't stop here!"}
                        Some(TileType::Rocks) => {"Rocks: Dangerous. Carts break on the rocks!"}
                        Some(TileType::MuddyRocks) => {"Muddy Rocks: Slippery AND Dangerous! Uh oh!"}
                        Some(TileType::Ditch) => {"Ditches: Dangerous. It's too deep for most things!"}
                        Some(TileType::ChickenPen) => {"A comfortable coop for the Chicken!"}
                        Some(TileType::HorsePen) => {"A nice stable for the Horse."}
                        Some(TileType::PigPen) => {"The Pig loves the Mud here."}
                        Some(TileType::GoatPen) => {"The Fences are extra sturdy for the Goat."}
                        Some(TileType::Corral) => {"A place for Cart maintenance and upkeep."}
                        _ => {""}
                    }.to_owned();
                }
                /*
                if buttons.just_pressed(MouseButton::Left) {
                    match field.get_tile_type(tile_pos_x, tile_pos_y, &q_tile) {
                        Some(TileType::Grass) => {field.set_tile(&mut commands, &sprites, TileType::Fence, tile_pos_x, tile_pos_y);}
                        Some(TileType::Fence) => {field.set_tile(&mut commands, &sprites, TileType::Mud, tile_pos_x, tile_pos_y);}
                        Some(TileType::Mud) => {field.set_tile(&mut commands, &sprites, TileType::Rocks, tile_pos_x, tile_pos_y);}
                        Some(TileType::Rocks) => {field.set_tile(&mut commands, &sprites, TileType::MuddyRocks, tile_pos_x, tile_pos_y);}
                        Some(TileType::MuddyRocks) => {field.set_tile(&mut commands, &sprites, TileType::Corral, tile_pos_x, tile_pos_y);}
                        Some(TileType::Corral) => {field.set_tile(&mut commands, &sprites, TileType::ChickenPen, tile_pos_x, tile_pos_y);}
                        Some(TileType::ChickenPen) => {field.set_tile(&mut commands, &sprites, TileType::HorsePen, tile_pos_x, tile_pos_y);}
                        Some(TileType::HorsePen) => {field.set_tile(&mut commands, &sprites, TileType::PigPen, tile_pos_x, tile_pos_y);}
                        Some(TileType::PigPen) => {field.set_tile(&mut commands, &sprites, TileType::GoatPen, tile_pos_x, tile_pos_y);}
                        Some(TileType::GoatPen) => {field.set_tile(&mut commands, &sprites, TileType::Ditch, tile_pos_x, tile_pos_y);}
                        Some(TileType::Ditch) => {field.set_tile(&mut commands, &sprites, TileType::Grass, tile_pos_x, tile_pos_y);}
                        _ => {}
                    }
                }
                if buttons.just_pressed(MouseButton::Right) {
                    match field.get_entity_type(tile_pos_x, tile_pos_y, q_entity) {
                        Some(EntityType::Chicken) => {field.set_entity(&mut commands, &sprites, EntityType::Horse, tile_pos_x, tile_pos_y);}
                        Some(EntityType::Horse) => {field.set_entity(&mut commands, &sprites, EntityType::Pig, tile_pos_x, tile_pos_y);}
                        Some(EntityType::Pig) => {field.set_entity(&mut commands, &sprites, EntityType::Goat, tile_pos_x, tile_pos_y);}
                        Some(EntityType::Goat) => {field.set_entity(&mut commands, &sprites, EntityType::Wagon, tile_pos_x, tile_pos_y);}
                        Some(EntityType::Wagon) => {field.set_entity(&mut commands, &sprites, EntityType::ChickenFood, tile_pos_x, tile_pos_y);}
                        Some(EntityType::ChickenFood) => {field.set_entity(&mut commands, &sprites, EntityType::HorseFood, tile_pos_x, tile_pos_y);}
                        Some(EntityType::HorseFood) => {field.set_entity(&mut commands, &sprites, EntityType::PigFood, tile_pos_x, tile_pos_y);}
                        Some(EntityType::PigFood) => {field.set_entity(&mut commands, &sprites, EntityType::AllFood, tile_pos_x, tile_pos_y);}
                        Some(EntityType::AllFood) => {field.set_entity(&mut commands, &sprites, EntityType::None, tile_pos_x, tile_pos_y);}
                        //Some(EntityType::WagonFood) => {field.set_entity(&mut commands, &sprites, EntityType::WagonAnimal, tile_pos_x, tile_pos_y);}
                        //Some(EntityType::WagonAnimal) => {field.set_entity(&mut commands, &sprites, EntityType::None, tile_pos_x, tile_pos_y);}
                        _ => {field.set_entity(&mut commands, &sprites, EntityType::Chicken, tile_pos_x, tile_pos_y);}
                    }
                }
                */
                if let Ok(mut cursor) = q_transform.get_mut(field.cursor) {
                    cursor.scale = Vec3::splat(ui_scale.scale as f32);
                    cursor.translation = Vec3{ x: (tile.x.floor() + 0.5) * TILE_SIZE * cursor.scale.x, y: tile.y.round() * TILE_SIZE * cursor.scale.y, z:10.0 };
                }
            }
        }
    }
}

pub fn saving_system(
    mut commands: Commands, 
    sprites: Res<Sprites>,
    levels: Res<Levels>,
    savefiles: Res<Assets<SaveFile>>,
    mut field: ResMut<Field>, 
    q_tile: Query<&Tile>,
    q_entity: Query<&GameEntity>,
    mut simulation: ResMut<SimulateRes>,
    mut saving: ResMut<SaveRes>,){
    match saving.saving {
        SaveStage::Saving => {
            if field.tiles.len() <= 0 || field.tiles[0].len() <= 0 {
                println!("You FOOL! There is no level to save!");
                return;
            }
            let mut save = SaveFile { version: 2, width: field.tiles.len(), height: field.tiles[0].len(), tiles: vec![] };

            let mut y = 0;
            while y < save.height {
                let mut x = 0;
                while x < save.width {
                    let mut save_tile: Option<Tile> = None;
                    let mut save_entity_1: Option<GameEntity> = None;
                    let mut save_entity_2: Option<GameEntity> = None;
                    let mut save_entity_3: Option<GameEntity> = None;
                    if let Ok(tile) = q_tile.get(field.tiles[x][y].0) {
                        save_tile = Some(tile.clone());
                    }
                    if let Some(entity_id) = field.tiles[x][y].1 {
                        if let Ok(entity) = q_entity.get(entity_id) {
                            save_entity_1 = Some(entity.clone());
                        }
                    }
                    if let Some(entity_id) = field.tiles[x][y].2 {
                        if let Ok(entity) = q_entity.get(entity_id) {
                            save_entity_2 = Some(entity.clone());
                        }
                    }
                    if let Some(entity_id) = field.tiles[x][y].3 {
                        if let Ok(entity) = q_entity.get(entity_id) {
                            save_entity_3 = Some(entity.clone());
                        }
                    }
                    save.tiles.push((save_tile, save_entity_1, save_entity_2, save_entity_3));
                    x += 1;
                }
                y += 1;
            }

            if let Ok(save_string) = serde_json::to_string(&save){
                let _ = fs::write("level.skb", save_string);
                saving.save = "level.skb".to_owned();
            }

            saving.saving = SaveStage::Idle;
        }
        SaveStage::Loading => {
            println!("LOADING {}", saving.save.to_owned());
            /*if let Ok(save_string) = fs::read_to_string(saving.save.to_owned()) {
                if let Ok(save) = serde_json::from_str::<SaveFile>(&save_string) {
                    simulation.rounds = 1;
                    for savetile in save.tiles {
                        if let Some(tile) = savetile.0 {
                            field.set_tile(&mut commands, &sprites, tile.tile_type, tile.location.x, tile.location.y);
                            field.set_entity(&mut commands, &sprites, EntityType::None, tile.location.x, tile.location.y);
                        }
                        if let Some(entity) = savetile.1 {
                            field.set_entity(&mut commands, &sprites, entity.entity_type, entity.location.x, entity.location.y);
                        }
                        if let Some(entity) = savetile.2 {
                            field.set_entity(&mut commands, &sprites, entity.entity_type, entity.location.x, entity.location.y);
                        }
                        if let Some(entity) = savetile.3 {
                            field.set_entity(&mut commands, &sprites, entity.entity_type, entity.location.x, entity.location.y);
                        }
                    }
                }else if let Err(error) = serde_json::from_str::<SaveFile>(&save_string){
                    println!("Level Loading Failed! Error: {:?}", error);
                }
            }else */ {
                    println!("{:?}", &levels.levels[&saving.save]);
                    println!("{:?}", savefiles.get(&levels.levels[&saving.save]));
                if let Some(save) = savefiles.get(&levels.levels[&saving.save]) {
                    println!("?");
                    simulation.rounds = 1;
                    for savetile in &save.tiles {
                        if let Some(tile) = savetile.0 {
                            field.set_tile(&mut commands, &sprites, tile.tile_type, tile.location.x, tile.location.y);
                            field.set_entity(&mut commands, &sprites, EntityType::None, tile.location.x, tile.location.y);
                        }
                        if let Some(entity) = savetile.1 {
                            field.set_entity(&mut commands, &sprites, entity.entity_type, entity.location.x, entity.location.y);
                        }
                        if let Some(entity) = savetile.2 {
                            field.set_entity(&mut commands, &sprites, entity.entity_type, entity.location.x, entity.location.y);
                        }
                        if let Some(entity) = savetile.3 {
                            field.set_entity(&mut commands, &sprites, entity.entity_type, entity.location.x, entity.location.y);
                        }
                    }
                }
            }

            saving.saving = SaveStage::Idle;
        }
        _ => {}
    }
}