
use bevy::prelude::*;
use crate::*;

#[derive(Resource)]
#[derive(Default)]
pub struct Weather {
    pub weather: WeatherType,
    pub raindrops: u128,
    pub raindrop_count: u128,
    pub overlay: Option<Entity>
}


#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[allow(dead_code)]
pub enum WeatherType {
    #[default] Sunny,
    Cloudy,
    Raining,
    Night,
    RainyNight,
    Thunder
}

#[derive(Component, Deref, DerefMut)]
pub struct Raindrop(Timer);

pub fn weather_system(mut commands: Commands, 
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut rain_q: Query<(Entity, &mut TextureAtlasSprite, &mut Scaling, &mut Raindrop)>,
    mut sprite_q: Query<&mut Sprite>,
    mut weather: ResMut<Weather>,) {
    match weather.weather {
        WeatherType::Raining => {
            if let Some(overlay_id) = weather.overlay {
                if let Ok(mut overlay) = sprite_q.get_mut(overlay_id) {
                    overlay.color = Color::rgba(0.05, 0.05, 0.25, 0.25);
                }
            }
            weather.raindrops += time.delta().as_micros();
            let mut raindrops = weather.raindrops / weather.raindrop_count;
            weather.raindrops = weather.raindrops % weather.raindrop_count;
            while raindrops > 0 {
                commands.spawn((SpriteSheetBundle {
                    texture_atlas: sprites.sprites["Rain"].clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_xyz(0.0, 0.0, 110.0),
                    ..default()
                }, Raindrop(Timer::from_seconds(((rng.next_u32() % 600) as f32) / 100.0, TimerMode::Once)), 
                Scaling { position: Vec2{ x: ((rng.next_u32() % 1600) as f32) / 100.0, y: 10.0 + ((((rng.next_u32() % 200) as i32) - 100) as f32) / 100.0 } }));
                raindrops -= 1;
            }

            for (raindrop, mut raindrop_sprite, mut raindrop_position, mut raindrop_timer) in &mut rain_q {
                if raindrop_sprite.index == 0 {
                    raindrop_position.position += Vec2 { x:-0.02, y:-0.15 };
                }
                raindrop_timer.tick(time.delta());
                if raindrop_timer.just_finished() {
                    if raindrop_sprite.index == 0 {
                        *raindrop_timer = Raindrop(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating));
                    }
                    raindrop_sprite.index += 1;
                    if raindrop_sprite.index >= 4 {
                        commands.entity(raindrop).despawn();
                    }
                }
            }
        }
        WeatherType::Night => {
            if let Some(overlay_id) = weather.overlay {
                if let Ok(mut overlay) = sprite_q.get_mut(overlay_id) {
                    overlay.color = Color::rgba(0.05, 0.05, 0.25, 0.6);
                }
            }

            for (raindrop, _, _, _) in &rain_q {
                commands.entity(raindrop).despawn();
            }
        }
        WeatherType::RainyNight => {
            if let Some(overlay_id) = weather.overlay {
                if let Ok(mut overlay) = sprite_q.get_mut(overlay_id) {
                    overlay.color = Color::rgba(0.025, 0.025, 0.15, 0.75);
                }
            }
            
            weather.raindrops += time.delta().as_micros();
            let mut raindrops = weather.raindrops / weather.raindrop_count;
            weather.raindrops = weather.raindrops % weather.raindrop_count;
            while raindrops > 0 {
                commands.spawn((SpriteSheetBundle {
                    texture_atlas: sprites.sprites["Rain"].clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_xyz(0.0, 0.0, 110.0),
                    ..default()
                }, Raindrop(Timer::from_seconds(((rng.next_u32() % 600) as f32) / 100.0, TimerMode::Once)), 
                Scaling { position: Vec2{ x: ((rng.next_u32() % 1600) as f32) / 100.0, y: 10.0 + ((((rng.next_u32() % 200) as i32) - 100) as f32) / 100.0 } }));
                raindrops -= 1;
            }

            for (raindrop, mut raindrop_sprite, mut raindrop_position, mut raindrop_timer) in &mut rain_q {
                if raindrop_sprite.index == 0 {
                    raindrop_position.position += Vec2 { x:-0.02, y:-0.15 };
                }
                raindrop_timer.tick(time.delta());
                if raindrop_timer.just_finished() {
                    if raindrop_sprite.index == 0 {
                        *raindrop_timer = Raindrop(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating));
                    }
                    raindrop_sprite.index += 1;
                    if raindrop_sprite.index >= 4 {
                        commands.entity(raindrop).despawn();
                    }
                }
            }
        }
        _ => {
            if let Some(overlay_id) = weather.overlay {
                if let Ok(mut overlay) = sprite_q.get_mut(overlay_id) {
                    overlay.color = Color::rgba(0.05, 0.05, 0.25, 0.0);
                }
            }

            for (raindrop, _, _, _) in &rain_q {
                commands.entity(raindrop).despawn();
            }
        }
    }
    if weather.overlay.is_none() {
        weather.overlay = Some(
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.05, 0.05, 0.25),
                    custom_size: Some(Vec2::new(32.0*14.0, 32.0*8.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 90.0),
                ..default()
            },
            Scaling {
                position: Vec2::new(6.5,3.5)
            }
        )).id());
    }
}