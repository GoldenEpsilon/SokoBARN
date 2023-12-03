mod game;
mod menu;
mod simulation;
mod weather;

use crate::game::*;
use crate::menu::*;
use crate::simulation::*;
use crate::weather::*;
use bevy::audio::PlaybackMode;
use bevy::ecs::schedule::common_conditions;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_rand::prelude::*;
use rand_core::RngCore;
use bevy_prng::ChaCha8Rng;

static TILE_SIZE: f32 = 32.0;
static ASPECT_RATIO_W: f32 = 16.0;
static ASPECT_RATIO_H: f32 = 9.0;
static TILE_OFFSET_X: f32 = 7.5;
static TILE_OFFSET_Y: f32 = 3.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    LevelSelect,
    Gameplay,
    Pause,
}

#[derive(Component)]
pub struct Cursor {
    pub holding: EntityType,
    pub drag_drop: bool,
    pub starting_pos: Vec2,
    pub pos: Vec2
}

pub static CURSOR_MIN_MOVE_DIST: f32 = 12.0;

#[derive(Component)]
pub struct CursorObj {
    index: usize
}

#[derive(Component)]
pub struct KeyArt;

#[derive(Component)]
pub struct Scaling {
    position: Vec2
}

#[derive(Resource)]
#[derive(Default)]
pub struct Sprites {
    sprites: HashMap<String, Handle<TextureAtlas>>
}

#[derive(Resource)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct ReloadLevelSelect {
    reloading: bool
}

#[derive(Resource)]
#[derive(Default)]
pub struct UIImages {
    sprites: HashMap<String, Handle<Image>>
}

#[derive(Resource)]
#[derive(Default)]
pub struct Sounds {
    sounds: HashMap<String, Handle<AudioSource>>
}

#[derive(Resource)]
#[derive(Default)]
pub struct Levels {
    levels: HashMap<String, Handle<SaveFile>>
}

fn main() {
    let mut app = App::new();

    app
        .add_state::<GameState>()
        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire window
                    fit_canvas_to_parent: true,
                    //resolution: (512., 288.).into(),
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }).set(ImagePlugin::default_nearest()),
            JsonAssetPlugin::<SaveFile>::new(&["skb"]),
            EntropyPlugin::<ChaCha8Rng>::default()
        ))
        .insert_resource(ClearColor(Color::hex("ACD132").unwrap()))
        .add_systems(Startup, setup)

        //Menus
        .add_systems(OnEnter(GameState::Menu), (game_cleanup.run_if(resource_exists::<Field>()), menu_setup).chain())
        .add_systems(OnExit(GameState::Menu), menu_cleanup)

        //Menus
        .add_systems(OnEnter(GameState::LevelSelect), level_select_setup)
        .add_systems(Update, (menu_cleanup, level_select_setup).chain().run_if(in_state(GameState::LevelSelect).and_then(resource_equals(ReloadLevelSelect{reloading: true}))))
        .add_systems(OnExit(GameState::LevelSelect), menu_cleanup)
        
        .add_systems(OnEnter(GameState::Pause), pause_menu_setup)
        .add_systems(OnExit(GameState::Pause), pause_menu_cleanup)

        //Buttons
        .add_systems(Update, (button_system, button_update_system))

        //Move custom cursor
        .add_systems(Update, cursor)

        //Gameplay
        .add_systems(OnEnter(GameState::Gameplay), (setup_level, game_ui_setup).run_if(common_conditions::not(resource_exists::<Field>())))
        .add_systems(Update, saving_system.run_if(in_state(GameState::Gameplay).or_else(in_state(GameState::Pause))))
        .add_systems(Update, simulate.run_if(in_state(GameState::Gameplay)))
        .add_systems(Update, weather_system.run_if(in_state(GameState::Gameplay)))

        //Cursor Controls
        .add_systems(Update, (mouse_controls, apply_deferred).chain().run_if(in_state(GameState::Gameplay)))

        //Post Update Visuals
        .add_systems(PostUpdate, ((ditch_system, fence_system).run_if(in_state(GameState::Gameplay).or_else(in_state(GameState::Pause))), animation_system, effect_system, resize_system, apply_deferred).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {

    commands.insert_resource(SaveRes { saving: SaveStage::Idle, save: "level.skb".to_owned(), quicksaves: vec![], ..default() });
    commands.insert_resource(SimulateRes { simulating: false, rounds: 0, ..default() });
    commands.insert_resource(MenuData { button_entities: vec![] });
    commands.insert_resource(PauseMenuData { button_entities: vec![], mode: PauseMenuMode::Pause });
    commands.insert_resource(Weather { raindrop_count: 800 /*400*/, ..default() });
    commands.insert_resource(ReloadLevelSelect{reloading: true});

    let mut worlds = vec![];

    worlds.push(LevelWorld{
        name: "Editor".to_owned(),
        levels: vec![
            LevelData {
                name: "Blank".to_owned(),
                id: "Levels/blank.skb".to_owned(),
                editor: true,
                ..default()
            }
        ]
    });
    worlds.push(LevelWorld{
        name: "Tutorials".to_owned(),
        levels: vec![
            LevelData {
                name: "Goat 1".to_owned(),
                id: "Levels/goat-tutorial-1.skb".to_owned(),
                ..default()
            },
            LevelData {
                name: "Goat 2".to_owned(),
                id: "Levels/goat-tutorial-2.skb".to_owned(),
                ..default()
            },
            LevelData {
                name: "Horse 1".to_owned(),
                id: "Levels/horse-tutorial-1.skb".to_owned(),
                ..default()
            },
            LevelData {
                name: "Horse 2".to_owned(),
                id: "Levels/horse-tutorial-2.skb".to_owned(),
                ..default()
            }
        ]
    });
    worlds.push(LevelWorld{
        name: "Tutorials 2".to_owned(),
        levels: vec![
            LevelData {
                name: "Pig 1".to_owned(),
                id: "Levels/pig-tutorial-1.skb".to_owned(),
                ..default()
            },
            LevelData {
                name: "Pig 2".to_owned(),
                id: "Levels/pig-tutorial-2.skb".to_owned(),
                ..default()
            },
            LevelData {
                name: "Chicken 1".to_owned(),
                id: "Levels/chicken-tutorial-1.skb".to_owned(),
                ..default()
            },
            LevelData {
                name: "Chicken 2".to_owned(),
                id: "Levels/chicken-tutorial-2.skb".to_owned(),
                ..default()
            }
        ]
    });
    worlds.push(LevelWorld{
        name: "Extras".to_owned(),
        levels: vec![
            LevelData {
                name: "Rain 1".to_owned(),
                id: "Levels/Rain-1.skb".to_owned(),
                weather: WeatherType::Raining,
                ..default()
            },
            LevelData {
                name: "Rain 2".to_owned(),
                id: "Levels/Rain-2.skb".to_owned(),
                weather: WeatherType::Raining,
                ..default()
            },
            LevelData {
                name: "Night 1".to_owned(),
                id: "Levels/Night-1.skb".to_owned(),
                weather: WeatherType::Night,
                ..default()
            },
        ]
    });

    commands.insert_resource(WorldList { index: 0, worlds });

    let camera_bundle = Camera2dBundle::default();
    //camera_bundle.projection.scaling_mode = ScalingMode::Fixed { width: 640.0, height: 360.0 };
    commands.spawn(camera_bundle);

    
    let mut levels: HashMap<String, Handle<SaveFile>> = HashMap::new();
    {let level = "Levels/goat-tutorial-1.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/goat-tutorial-2.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/horse-tutorial-1.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/horse-tutorial-2.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/pig-tutorial-1.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/pig-tutorial-2.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/chicken-tutorial-1.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/chicken-tutorial-2.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/Night-1.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/Rain-1.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/Rain-2.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    {let level = "Levels/blank.skb";levels.insert(level.to_owned(), asset_server.load(level));}
    
    commands.insert_resource(Levels { levels: levels });

    
    let mut ui_images: HashMap<String, Handle<Image>> = HashMap::new();
    ui_images.insert("UISign".to_owned(), asset_server.load("Sprites/Misc/sokobarn-Signage.png"));
    ui_images.insert("UILogo".to_owned(), asset_server.load("Sprites/Misc/sokobarn-Logo.png"));
    ui_images.insert("UIBottom".to_owned(), asset_server.load("Sprites/Misc/sokobarn-lower-ui.png"));
    ui_images.insert("UIRight".to_owned(), asset_server.load("Sprites/Misc/sokobarn-level-ui-1.png"));
    commands.insert_resource(UIImages { sprites: ui_images });

    let mut sprites: HashMap<String, Handle<TextureAtlas>> = HashMap::new();
    sprites.insert("Chicken".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Animals/sokobarn-Chicken.png"), Vec2::new(28.0, 28.0), 4, 7, None, None)));
    sprites.insert("Pig".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Animals/sokobarn-Pig.png"), Vec2::new(28.0, 28.0), 4, 7, None, None)));
    sprites.insert("Horse".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Animals/sokobarn-Horse.png"), Vec2::new(28.0, 28.0), 4, 7, None, None)));
    sprites.insert("Goat".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Animals/sokobarn-Goat.png"), Vec2::new(28.0, 28.0), 4, 7, None, None)));
    sprites.insert("Wagon".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Animals/sokobarn-Empty-Cart.png"), Vec2::new(28.0, 28.0), 4, 7, None, None)));
    sprites.insert("Grass".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Grass-NEW.png"), Vec2::new(32.0, 32.0), 2, 2, None, None)));
    sprites.insert("Fence".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Fences.png"), Vec2::new(32.0, 32.0), 5, 1, None, None)));
    sprites.insert("Rocks".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Rocks.png"), Vec2::new(64.0, 64.0), 2, 2, None, None)));
    sprites.insert("Mud".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Mud.png"), Vec2::new(64.0, 64.0), 2, 2, None, None)));
    sprites.insert("MuddyRocks".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Muddy-Rocks.png"), Vec2::new(64.0, 64.0), 2, 2, None, None)));
    sprites.insert("Ditch".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Ditches.png"), Vec2::new(32.0, 32.0), 8, 2, None, None)));
    sprites.insert("Pens".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Tiles/sokobarn-Pens.png"), Vec2::new(48.0, 48.0), 5, 3, None, None)));
    sprites.insert("Food".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Food.png"), Vec2::new(28.0, 28.0), 5, 1, None, None)));
    sprites.insert("Cursor".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Cursors.png"), Vec2::new(64.0, 64.0), 5, 1, None, None)));
    sprites.insert("Rain".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Rain.png"), Vec2::new(5.0, 5.0), 4, 1, None, None)));
    sprites.insert("MuddySplash".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-MuddySplash.png"), Vec2::new(28.0, 28.0), 4, 1, None, None)));
    sprites.insert("Disabled".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Disabled.png"), Vec2::new(32.0, 32.0), 1, 1, None, None)));
    sprites.insert("Arrow".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Arrow.png"), Vec2::new(32.0, 32.0), 4, 1, None, None)));
    sprites.insert("Flags".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Flags.png"), Vec2::new(32.0, 32.0), 4, 24, None, None)));
    sprites.insert("Medals".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-level-medals.png"), Vec2::new(36.0, 36.0), 4, 1, None, None)));
    sprites.insert("Working".to_owned(), texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-working.png"), Vec2::new(28.0, 28.0), 2, 1, None, None)));

    commands.insert_resource(Sprites { sprites: sprites });

    let mut sounds: HashMap<String, Handle<AudioSource>> = HashMap::new();
    sounds.insert("Chicken1".to_owned(), asset_server.load("Sounds/Chicken1.ogg"));
    sounds.insert("Chicken2".to_owned(), asset_server.load("Sounds/Chicken2.ogg"));
    sounds.insert("Chicken3".to_owned(), asset_server.load("Sounds/Chicken3.ogg"));
    sounds.insert("Chicken4".to_owned(), asset_server.load("Sounds/Chicken4.ogg"));
    sounds.insert("Horse1".to_owned(), asset_server.load("Sounds/Horse1.ogg"));
    sounds.insert("Horse2".to_owned(), asset_server.load("Sounds/Horse2.ogg"));
    sounds.insert("Horse3".to_owned(), asset_server.load("Sounds/Horse3.ogg"));
    sounds.insert("Horse4".to_owned(), asset_server.load("Sounds/Horse4.ogg"));
    sounds.insert("Pig1".to_owned(), asset_server.load("Sounds/Pig1.ogg"));
    sounds.insert("Pig2".to_owned(), asset_server.load("Sounds/Pig2.ogg"));
    sounds.insert("Pig3".to_owned(), asset_server.load("Sounds/Pig3.ogg"));
    sounds.insert("Pig4".to_owned(), asset_server.load("Sounds/Pig4.ogg"));
    sounds.insert("Goat1".to_owned(), asset_server.load("Sounds/Goat1.ogg"));
    sounds.insert("Goat2".to_owned(), asset_server.load("Sounds/Goat2.ogg"));
    sounds.insert("Goat3".to_owned(), asset_server.load("Sounds/Goat3.ogg"));
    sounds.insert("Goat4".to_owned(), asset_server.load("Sounds/Goat4.ogg"));
    sounds.insert("Cart1".to_owned(), asset_server.load("Sounds/HorseAttach1.ogg"));
    sounds.insert("Cart2".to_owned(), asset_server.load("Sounds/HorseAttach2.ogg"));
    sounds.insert("Cart3".to_owned(), asset_server.load("Sounds/HorseAttach3.ogg"));
    sounds.insert("Mud1".to_owned(), asset_server.load("Sounds/Mud1.ogg"));
    sounds.insert("Mud2".to_owned(), asset_server.load("Sounds/Mud2.ogg"));
    sounds.insert("Mud3".to_owned(), asset_server.load("Sounds/Mud3.ogg"));
    sounds.insert("Mud4".to_owned(), asset_server.load("Sounds/Mud4.ogg"));
    sounds.insert("GoatCrash".to_owned(), asset_server.load("Sounds/GoatCrash.ogg"));
    sounds.insert("ChickenFly1".to_owned(), asset_server.load("Sounds/ChickenFly1.ogg"));
    sounds.insert("ChickenFly2".to_owned(), asset_server.load("Sounds/ChickenFly2.ogg"));
    sounds.insert("ChickenFly3".to_owned(), asset_server.load("Sounds/ChickenFly3.ogg"));
    sounds.insert("ChickenFly4".to_owned(), asset_server.load("Sounds/ChickenFly4.ogg"));
    commands.insert_resource(Sounds { sounds });
    
    
    commands.spawn((NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            ..default()
        },
        z_index: ZIndex::Global(15),
        ..default()
    }, Cursor{holding: EntityType::None, drag_drop: true, starting_pos: Vec2::splat(-100.0), pos: Vec2::splat(-100.0)})
    ).with_children(|parent| {
        parent.spawn((AtlasImageBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Flags.png"), Vec2::new(32.0, 32.0), 4, 24, None, None)),
            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(24.0),
                left: Val::Px(16.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }, CursorObj{index:2}));
        parent.spawn((AtlasImageBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Food.png"), Vec2::new(28.0, 28.0), 5, 1, None, None)),
            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(24.0),
                left: Val::Px(16.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        }, CursorObj{index:1}));
        parent.spawn((AtlasImageBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(asset_server.load("Sprites/Misc/sokobarn-Cursors.png"), Vec2::new(64.0, 64.0), 5, 1, None, None)),
            texture_atlas_image: UiTextureAtlasImage{index:0,..default()},
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        }, CursorObj{index:0}));
    });

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_content: AlignContent::Center,
            ..default()
        },
        z_index: ZIndex::Global(15),
        ..default()
    }
    ).with_children(|parent| {
        parent.spawn((ImageBundle {
            image: UiImage::new(asset_server.load("UIKeyArt.png").clone()),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(512.0),
                height: Val::Px(288.0),
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            z_index: ZIndex::Global(-1),
            background_color: Color::WHITE.into(),
            ..Default::default()
        }, KeyArt));
    });

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
        Query<(&mut Transform, &Depth)>,
        Query<(&mut Transform, &Scaling)>,)>,
    windows: Query<&Window>,
    mut ui_scale: ResMut<UiScale>,){
    for window in &windows {
        let size = (window.width()/ASPECT_RATIO_W).min(window.height()/ASPECT_RATIO_H)/TILE_SIZE;
        for (mut transform, game_entity) in &mut object_set.p0().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = transform.translation.lerp(Vec3{ 
                x: (game_entity.location.x as f32 - TILE_OFFSET_X)*TILE_SIZE*size, 
                y: (game_entity.location.y as f32 - TILE_OFFSET_Y)*TILE_SIZE*size, 
                z: -(game_entity.location.y as f32) * 4.0 + -(game_entity.location.x as f32)*0.1 + game_entity.location.z as f32 
            }, 0.2);
        }
        for (mut transform, tile) in &mut object_set.p1().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = Vec3{ 
                x: (tile.location.x as f32 - TILE_OFFSET_X)*TILE_SIZE*size, 
                y: (tile.location.y as f32 - TILE_OFFSET_Y)*TILE_SIZE*size, 
                z: -(tile.location.y as f32) * 4.0 + -(tile.location.x as f32)*0.1 + tile.location.z as f32 
            };
        }
        for (mut transform, depth) in &mut object_set.p2().iter_mut() {
            transform.translation = Vec3{ 
                x: transform.translation.x, 
                y: transform.translation.y, 
                z: depth.depth/size
            };
        }
        for (mut transform, scaling_obj) in &mut object_set.p3().iter_mut() {
            transform.scale = Vec3::splat(size);
            transform.translation = Vec3{ 
                x: (scaling_obj.position.x as f32 - TILE_OFFSET_X)*TILE_SIZE*size, 
                y: (scaling_obj.position.y as f32 - TILE_OFFSET_Y)*TILE_SIZE*size, 
                z: transform.translation.z
            };
        }
        ui_scale.scale = size as f64;
    }
}

pub fn cursor(
    q_windows: Query<&Window, With<PrimaryWindow>>, 
    mut q_cursor: Query<(&mut Cursor, &mut Style, &Children)>, 
    mut q_held_item: Query<(&CursorObj, &mut UiTextureAtlasImage, &mut Visibility)>, 
    buttons: Res<Input<MouseButton>>,
    ui_scale: Res<UiScale>,){
    if let Ok(window) = q_windows.get_single() {
        if let Some(position) = window.cursor_position() {
            if let Ok((mut cursor, mut style, children)) = q_cursor.get_single_mut() {
                style.left = Val::Px((position.x-64.0-20.0)/(ui_scale.scale as f32));
                style.top = Val::Px((position.y-64.0-20.0)/(ui_scale.scale as f32));

                cursor.pos = position;
                if Vec2::distance(cursor.pos, cursor.starting_pos) > CURSOR_MIN_MOVE_DIST {
                    cursor.starting_pos = Vec2::splat(-100.0);
                }
                
                for &child in children.iter() {
                    if let Ok((obj_type, mut sprite, mut visible)) = q_held_item.get_mut(child) {
                        match obj_type.index {
                            0 => {
                                if cursor.holding == EntityType::None {
                                    if buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right) {
                                        sprite.index = 1;
                                    }else{
                                        sprite.index = 0;
                                    }
                                } else {
                                    if (buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right)) == cursor.drag_drop {
                                        sprite.index = 3;
                                    }else{
                                        sprite.index = 2;
                                    }
                                }
                            }
                            1 => {
                                *visible = Visibility::Visible;
                                match cursor.holding {
                                    EntityType::ChickenFood => {sprite.index = 0;}
                                    EntityType::HorseFood => {sprite.index = 1;}
                                    EntityType::PigFood => {sprite.index = 2;}
                                    EntityType::AllFood => {sprite.index = 3;}
                                    EntityType::WagonFood => {sprite.index = 4;}
                                    _ => {
                                        *visible = Visibility::Hidden;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn animation_system(
    mut q_entities: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut GameEntity)>,
    time: Res<Time>,){
    for (mut sprite, mut timer, mut entity) in &mut q_entities {
        timer.tick(time.delta());
        if timer.just_finished() {
            let offset = 4 *
            match entity.state {
                EntityState::Idle => {0}
                EntityState::Walking => {1}
                EntityState::Sliding => {2}
                EntityState::Eating => {4}
                EntityState::Celebrating => {5}
                EntityState::Special => {6}
                EntityState::Failure => {2}
            };
            sprite.index = (sprite.index + 1) %  
            match entity.state {
                EntityState::Idle => {2}
                EntityState::Walking => {4}
                EntityState::Sliding => {4}
                EntityState::Eating => {2}
                EntityState::Celebrating => {4}
                EntityState::Special => {2}
                EntityState::Failure => {if entity.entity_type == EntityType::Wagon { 2 } else { 4 } }
            } + offset;
            if let Some(prev_state) = entity.prev_state {
                if prev_state != entity.state {
                    sprite.index = offset;
                }
                //Animation Finished!
                if prev_state == entity.state && sprite.index == offset {
                    if entity.state == EntityState::Special && entity.entity_type == EntityType::Goat {
                        entity.state = EntityState::Idle;
                    }
                    if entity.state == EntityState::Failure {
                        sprite.index = offset + 1;
                    }
                }
            }
            entity.prev_state = Some(entity.state.to_owned());
        }
    }
}

pub fn effect_system(
    mut commands: Commands,
    mut q_entities: Query<(Entity, &mut TextureAtlasSprite, &mut Effect)>,
    time: Res<Time>,){
    for (effect, mut sprite, mut timer) in &mut q_entities {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = sprite.index + 1;
            if sprite.index >= 4 {
                commands.entity(effect).despawn();
            }
        }
    }
}