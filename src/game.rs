use crate::*;

use bevy::prelude::*;

#[derive(Bundle)]
struct AnimalBundle {
    animal: Animal,
    location: Location,
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    location: Location,
    sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Animal;

#[derive(Resource)]
pub struct Field {
    tiles: Vec<Vec<Tile>>
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Location {
    pub position: Vec3
}

pub fn setup_level(mut commands: Commands, sprites: Res<Sprites>){
    let tiles = vec![];
    let mut x = -8.0;
    while x < 8.0 {
        let mut y = -4.0;
        while y < 5.0 {
            commands.spawn(
                TileBundle {
                    tile: Tile,
                    location: Location { 
                        position: Vec3 { x: x, y: y, z: 0.0 }
                    },
                    sprite: SpriteSheetBundle {
                        texture_atlas: sprites.sprites["Grass"].clone(),
                        sprite: TextureAtlasSprite::new(0),
                        ..default()
                    }
                }
            );
            y += 1.0;
        }
        x += 1.0;
    }
    
    commands.spawn(
        AnimalBundle {
            animal: Animal,
            location: Location { 
                position: Vec3 { x: 1.0, y: 1.0, z: 0.0 }
            },
            sprite: SpriteSheetBundle {
                texture_atlas: sprites.sprites["Pig"].clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_translation(Vec3 { x: 24.0 + 12.0, y: 24.0 , z: 1.0 }),
                ..default()
            }
        }
    );

    commands.insert_resource(Field { tiles: tiles });
}

