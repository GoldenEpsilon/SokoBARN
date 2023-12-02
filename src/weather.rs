
use bevy::prelude::*;
use crate::*;

#[derive(Resource)]
#[derive(Default)]
pub struct Weather {
    pub weather: WeatherType,
    pub raindrops: u128,
    pub raindrop_count: u128,
}


#[derive(PartialEq)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub enum WeatherType {
    #[default] Sunny,
    Cloudy,
    Raining,
    Night,
    Thunder
}

#[derive(Component, Deref, DerefMut)]
pub struct Raindrop(Timer);

pub fn weather_system(mut commands: Commands, 
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut rain_q: Query<(Entity, &mut TextureAtlasSprite, &mut Scaling, &mut Raindrop)>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut weather: ResMut<Weather>,) {
    
    if weather.weather == WeatherType::Raining {
        weather.raindrops += time.delta().as_micros();
        let mut raindrops = weather.raindrops / weather.raindrop_count;
        weather.raindrops = weather.raindrops % weather.raindrop_count;
        while raindrops > 0 {
            commands.spawn((SpriteSheetBundle {
                texture_atlas: sprites.sprites["Rain"].clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
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
}