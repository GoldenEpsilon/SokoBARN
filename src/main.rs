mod game;
mod menu;

use crate::game::*;
use crate::menu::*;
use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;

static TILE_SIZE: f32 = 32.0;
static ASPECT_RATIO_W: f32 = 16.0;
static ASPECT_RATIO_H: f32 = 9.0;
static TILE_OFFSET_X: f32 = 7.5;
static TILE_OFFSET_Y: f32 = 3.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Gameplay,
}

#[derive(Component)]
pub struct Cursor;

#[derive(Resource)]
#[derive(Default)]
pub struct Sprites {
    sprites: HashMap<String, Handle<TextureAtlas>>
}

fn main() {
    let mut app = App::new();

    app
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire window
                fit_canvas_to_parent: true,
                //resolution: (512., 288.).into(),
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)

        //Menus
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(Update, (button_system, button_update_system))
        .add_systems(OnExit(GameState::Menu), menu_cleanup)


        //Move custom cursor
        .add_systems(Update, cursor)

        //Gameplay
        .add_systems(OnEnter(GameState::Gameplay), (setup_level, game_ui_setup))
        .add_systems(Update, saving_system.run_if(in_state(GameState::Gameplay)))

        //Cursor Controls
        .add_systems(Update, mouse_controls.run_if(in_state(GameState::Gameplay)))

        //Post Update Visuals
        .add_systems(PostUpdate, (fence_system.run_if(in_state(GameState::Gameplay)), resize_system, apply_deferred).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {

    commands.insert_resource(SaveRes { saving: SaveStage::Idle });

    let camera_bundle = Camera2dBundle::default();
    //camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 640.0, height: 360.0 };
    commands.spawn(camera_bundle);


    let mut sprites: HashMap<String, Handle<TextureAtlas>> = HashMap::new();
    sprites.insert("Grass".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Grass.png"), Vec2::new(32.0, 32.0), 8, 1, None, None)));
    sprites.insert("Chicken".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Chicken.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Pig".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Pig.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Horse".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Horse.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Goat".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Goat.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Wagon".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Cart.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Food".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Food.png"), Vec2::new(28.0, 28.0), 5, 1, None, None)));
    sprites.insert("Fence".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Fences.png"), Vec2::new(32.0, 32.0), 5, 1, None, None)));
    sprites.insert("Rocks".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Rocks.png"), Vec2::new(64.0, 64.0), 2, 2, None, None)));
    sprites.insert("Mud".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Mud.png"), Vec2::new(64.0, 64.0), 2, 2, None, None)));
    sprites.insert("MuddyRocks".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/MuddyRocks.png"), Vec2::new(64.0, 64.0), 2, 2, None, None)));
    sprites.insert("Pens".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Pens.png"), Vec2::new(48.0, 48.0), 5, 3, None, None)));
    sprites.insert("Cursor".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Cursor.png"), Vec2::new(64.0, 64.0), 5, 1, None, None)));

    commands.insert_resource(Sprites { sprites: sprites });
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Cursor.png"), Vec2::new(64.0, 64.0), 5, 1, None, None)),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        }, Cursor));

    commands.spawn(AudioBundle {
        source: asset_server.load("Music/contemplativealgorithmic_demo_1.ogg"),
        settings: PlaybackSettings{
            mode: PlaybackMode::Loop,
            ..default()
        },
        ..default()
    });
}

fn resize_system(mut object_set: ParamSet<(
        Query<(&mut Transform, &GameEntity)>,
        Query<(&mut Transform, &Tile)>,
        Query<&mut Transform, With<Cursor>>)>,
    windows: Query<&Window>,
    mut ui_scale: ResMut<UiScale>,){
    for window in &windows {
        let size = (window.width()/ASPECT_RATIO_W).min(window.height()/ASPECT_RATIO_H)/TILE_SIZE;
        for (mut transform, game_entity) in &mut object_set.p0().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = Vec3{ 
                x: ((game_entity.location.x as f32 - TILE_OFFSET_X)*TILE_SIZE)*size, 
                y: (game_entity.location.y as f32 - TILE_OFFSET_Y)*TILE_SIZE*size, 
                z: -(game_entity.location.y as f32)*0.1 + -(game_entity.location.x as f32)*0.01 + game_entity.location.z as f32 
            };
        }
        for (mut transform, tile) in &mut object_set.p1().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = Vec3{ 
                x: ((tile.location.x as f32 - TILE_OFFSET_X)*TILE_SIZE)*size, 
                y: (tile.location.y as f32 - TILE_OFFSET_Y)*TILE_SIZE*size, 
                z: -(tile.location.y as f32)*0.1 + -(tile.location.x as f32)*0.01 + tile.location.z as f32 
            };
        }
        for mut transform in &mut object_set.p2().iter_mut() {
            transform.scale = Vec3::splat(size);
        }
        ui_scale.scale = size as f64;
    }
}

//TODO: make cursor a ui object
pub fn cursor(q_windows: Query<&Window, With<PrimaryWindow>>, mut q_cursor: Query<(&mut Transform, &mut TextureAtlasSprite), With<Cursor>>, buttons: Res<Input<MouseButton>>){
    if let Ok(window) = q_windows.get_single() {
        if let Some(position) = window.cursor_position() {
            if let Ok((mut cursor, mut sprite)) = q_cursor.get_single_mut() {
                cursor.translation = Vec3{ x: position.x - (window.width()/2.0), y: (window.height()/2.0) - position.y, z:10.0 };
                if buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right) {
                    sprite.index = 1;
                }else{
                    sprite.index = 0;
                }
            }
        }
    }
}