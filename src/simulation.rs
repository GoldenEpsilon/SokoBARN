use crate::*;
use crate::game::*;

use bevy::prelude::*;

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
    tile_q: Query<&Tile>,){
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
                            EntityType::Goat => {EntityState::Idle}
                            _ => {entity.state}
                        };
                    }
                    match entity.entity_type {
                        EntityType::Chicken | EntityType::Pig | EntityType::Horse | EntityType::Goat | EntityType::Wagon => {
                            match state {
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
                if field.check_win(&entity_q.to_readonly(), &tile_q) {
                    for mut entity in &mut entity_q {
                        entity.state = EntityState::Celebrating;
                    }
                    pause_menu_data.mode = PauseMenuMode::Win;
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