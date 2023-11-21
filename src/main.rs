mod game;
mod menu;

use crate::game::*;
use crate::menu::*;
use bevy::prelude::*;
//use bevy::render::camera::ScalingMode;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy::window::WindowResized;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Gameplay,
}

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
                // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)

        //Menus
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(Update, (button_system).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), menu_cleanup)

        //Resize Sprites
        .add_systems(Update, resize_system)

        //Gameplay
        .add_systems(OnEnter(GameState::Gameplay), setup_level)

        //Make fences connect
        .add_systems(Update, fence_system.run_if(in_state(GameState::Gameplay)))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {
    let camera_bundle = Camera2dBundle::default();
    //camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 640.0, height: 360.0 };
    commands.spawn(camera_bundle);


    let mut sprites: HashMap<String, Handle<TextureAtlas>> = HashMap::new();
    sprites.insert("Grass".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Grass.png"), Vec2::new(32.0, 32.0), 8, 1, None, None)));
    sprites.insert("Chicken".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Chicken.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Pig".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Pig.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Horse".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Horse.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Goat".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Goat.png"), Vec2::new(28.0, 28.0), 3, 7, None, None)));
    sprites.insert("Fence".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Fences.png"), Vec2::new(32.0, 32.0), 5, 1, None, None)));
    commands.insert_resource(Sprites { sprites: sprites });
    
    commands.spawn(AudioBundle {
        source: asset_server.load("Music/contemplativealgorithmic_demo_1.ogg"),
        ..default()
    });
}

fn resize_system(mut object_set: ParamSet<(
        Query<(&mut Transform, &Location)>,
        Query<(&mut Transform, &Tile)>)>,
    windows: Query<&Window>){
    for window in &windows {
        let tile_size = 32.0;
        let tile_offset_x = 7.5;
        let tile_offset_y = 4.0;
        let size = (window.width()/16.0).min(window.height()/9.0)/tile_size;
        for (mut transform, location) in &mut object_set.p0().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = Vec3{ x: ((location.x as f32 - tile_offset_x)*tile_size)*size, y: (location.y as f32 - tile_offset_y)*tile_size*size, z: location.z as f32 };
        }
        for (mut transform, tile) in &mut object_set.p1().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = Vec3{ x: ((tile.location.x as f32 - tile_offset_x)*tile_size)*size, y: (tile.location.y as f32 - tile_offset_y)*tile_size*size, z: tile.location.z as f32 };
        }
    }
}