use crate::*;
use crate::game::*;

use bevy::prelude::*;
use bevy_pkv::PkvStore;

pub fn simulate(
    mut commands: Commands, 
    mut field: ResMut<Field>,
    time: Res<Time>,
    sounds: Res<Sounds>,
    sprites: Res<Sprites>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
    mut simulating: ResMut<SimulateRes>,
    mut entity_q: Query<&mut GameEntity>,
    mut saving: ResMut<SaveRes>,
    mut next_state: ResMut<NextState<GameState>>,
    mut pause_menu_data: ResMut<PauseMenuData>,
    mut pkv: ResMut<PkvStore>,
    mut medals: ResMut<Medals>,
    mut working_q: Query<(&mut TextureAtlasSprite, &mut Visibility, &mut AnimationTimer)>,
    tile_q: Query<&Tile>,){
    if let Some(indicator) = simulating.indicator {
        if let Ok((mut tex, mut visible, mut timer)) = working_q.get_mut(indicator){
            if simulating.simulating {
                *visible = Visibility::Visible;
                timer.tick(time.delta());
                if timer.just_finished() {
                    tex.index = (tex.index + 1) % 2;
                }
            } else {
                *visible = Visibility::Hidden;
            }
        }
    }
    if simulating.indicator.is_none() {
        simulating.indicator = Some(
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: sprites.sprites["Working"].clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_xyz(0.0, 0.0, 150.0),
                ..default()
            },
            Scaling {
                position: Vec2::new(0.0, 7.0)
            },
            AnimationTimer(Timer::from_seconds(ANIMATION_SPEED, TimerMode::Repeating))
        )).id());
    }
    if simulating.loss {
        for mut entity in &mut entity_q {
            entity.state = EntityState::Failure;
        }
        pause_menu_data.mode = PauseMenuMode::Lose;
        saving.saving = SaveStage::SaveUndo;
        next_state.set(GameState::Pause);
        return;
    }
    if simulating.simulating && !simulating.loss && !simulating.win {
        field.simulate_timer.tick(time.delta());
        if field.simulate_timer.just_finished() {
            let mut full_simulation = false;
            if simulating.simulation_step == EntityType::None {
                simulating.simulation_step = EntityType::Goat;
                full_simulation = true;
            }
            println!("Simulation Tick!");
            let mut has_simulated = false;
            while simulating.simulation_step != EntityType::None && has_simulated != true {
                for entity in field.get_entities(&entity_q.to_readonly()) {
                    if entity.entity_type != simulating.simulation_step {
                        continue;
                    }
                    let mut state = entity.state;
                    if entity.state == EntityState::Special {
                        state = match entity.entity_type {
                            EntityType::Chicken => {EntityState::Sliding}
                            EntityType::Pig => {EntityState::Idle}
                            EntityType::Horse => {EntityState::Idle}
                            EntityType::Goat => {EntityState::Eating}
                            _ => {entity.state}
                        };
                    }
                    match entity.entity_type {
                        EntityType::Chicken | EntityType::Pig | EntityType::Horse | EntityType::Goat | EntityType::Wagon => {
                            match state {
                                EntityState::Eating => {
                                    if field.can_get_tile(entity.location.x, entity.location.y) {
                                        if let Some(entity) = field.tiles[entity.location.x][entity.location.y].3 {
                                            if let Ok(mut entity) = entity_q.get_mut(entity) {
                                                entity.state = EntityState::Idle;
                                                simulating.simulating = true;
                                                has_simulated = true;
                                            }
                                        }
                                    }
                                }
                                EntityState::Idle => {
                                    let target_location = field.can_see_food(entity, &entity_q.to_readonly(), &tile_q);
                                    if target_location.x != entity.location.x || target_location.y != entity.location.y {
                                        if !field.move_entity(&mut commands, &mut entity_q, &tile_q, &sounds, &sprites, &mut rng, entity, target_location) {
                                            simulating.simulating = false;
                                            simulating.loss = true;
                                            println!("FAIL STATE");
                                            return;
                                        }
                                        simulating.simulating = true;
                                        has_simulated = true;
                                        println!("{}", simulating.simulating);
                                    }
                                }
                                EntityState::Walking => {
                                    if !field.move_entity(&mut commands, &mut entity_q, &tile_q, &sounds, &sprites, &mut rng, entity, entity.target_location) {
                                        simulating.simulating = false;
                                        simulating.loss = true;
                                        println!("FAIL STATE");
                                        return;
                                    }
                                    simulating.simulating = true;
                                    has_simulated = true;
                                }
                                EntityState::Sliding => {
                                    if entity.last_direction != MoveDirection::None {
                                        if !field.slide_entity(&mut commands, &mut entity_q, &tile_q, &sounds, &sprites, &mut rng, entity, entity.last_direction) {
                                            simulating.simulating = false;
                                            simulating.loss = true;
                                            println!("FAIL STATE");
                                            return;
                                        }
                                        simulating.simulating = true;
                                        has_simulated = true;
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                if field.check_win(&mut entity_q, &tile_q) {
                    for mut entity in &mut entity_q {
                        entity.state = EntityState::Celebrating;
                    }
                    pause_menu_data.mode = PauseMenuMode::Win;
                    let mut earned_medal = 1;
                    if simulating.rounds <= field.par {
                        earned_medal = 2;
                    }
                    if simulating.rounds <= field.author_par {
                        earned_medal = 3;
                    }
                    if medals.medals[&field.level_id] < earned_medal {
                        *medals.medals.get_mut(&field.level_id).unwrap() = earned_medal;
                        pkv.set("save", &medals.to_owned()).expect("failed to store medals");
                    }
                    saving.saving = SaveStage::SaveUndo;
                    next_state.set(GameState::Pause);
                }
                simulating.simulation_step = match simulating.simulation_step {
                    EntityType::Goat => {EntityType::Horse}
                    EntityType::Horse => {EntityType::Pig}
                    EntityType::Pig => {EntityType::Chicken}
                    EntityType::Chicken => {EntityType::Wagon}
                    EntityType::Wagon => {EntityType::None}
                    _ => {EntityType::None}
                };
                if simulating.simulation_step == EntityType::None && !has_simulated && full_simulation {
                    simulating.simulating = false;
                }
            }
        }
        if !simulating.simulating {
            println!("Simulation Over!");
            saving.saving = SaveStage::SaveUndo;
        }
    }
    if !simulating.simulating && !simulating.loss {
        simulating.simulation_step = EntityType::None;
        /*if field.check_win(&entity_q.to_readonly(), &tile_q) {
            for mut entity in &mut entity_q {
                entity.state = EntityState::Celebrating;
            }
            simulating.win = true;
        }*/
    }
}