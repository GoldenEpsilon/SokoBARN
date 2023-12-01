use crate::*;
use crate::game::*;

use bevy::prelude::*;

pub fn simulate(
    mut commands: Commands, 
    mut field: ResMut<Field>,
    time: Res<Time>,
    mut simulating: ResMut<SimulateRes>,
    mut entity_q: Query<&mut GameEntity>,
    mut saving: ResMut<SaveRes>,
    tile_q: Query<&Tile>,){
    if simulating.simulating {
        field.simulate_timer.tick(time.delta());
        if field.simulate_timer.just_finished() {
            println!("Simulation Tick!");
            simulating.simulating = false;
            let mut celebrate = true;
            for entity in field.get_entities(&entity_q.to_readonly()) {
                match entity.entity_type {
                    EntityType::Chicken => {
                        if let Some(tile) = field.get_tile_type(entity.location.x, entity.location.y, &tile_q) {
                            if tile != TileType::ChickenPen {
                                celebrate = false;
                            }
                        } else {
                            celebrate = false;
                        }
                    }
                    EntityType::Pig => {
                        if let Some(tile) = field.get_tile_type(entity.location.x, entity.location.y, &tile_q) {
                            if tile != TileType::PigPen {
                                celebrate = false;
                            }
                        } else {
                            celebrate = false;
                        }
                    }
                    EntityType::Horse => {
                        if let Some(tile) = field.get_tile_type(entity.location.x, entity.location.y, &tile_q) {
                            if tile != TileType::HorsePen {
                                celebrate = false;
                            }
                        } else {
                            celebrate = false;
                        }
                    }
                    EntityType::Goat => {
                        if let Some(tile) = field.get_tile_type(entity.location.x, entity.location.y, &tile_q) {
                            if tile != TileType::GoatPen {
                                celebrate = false;
                            }
                        } else {
                            celebrate = false;
                        }
                    }
                    EntityType::Wagon => {
                        if let Some(tile) = field.get_tile_type(entity.location.x, entity.location.y, &tile_q) {
                            if tile != TileType::Corral {
                                celebrate = false;
                            }
                        } else {
                            celebrate = false;
                        }
                    }
                    _ => {}
                }
                let mut state = entity.state;
                if entity.state == EntityState::Special {
                    state = match entity.entity_type {
                        EntityType::Chicken => {EntityState::Walking}
                        EntityType::Pig => {EntityState::Idle}
                        EntityType::Horse => {EntityState::Idle}
                        EntityType::Goat => {EntityState::Idle}
                        _ => {entity.state}
                    };
                }
                match entity.entity_type {
                    EntityType::Chicken | EntityType::Pig | EntityType::Horse | EntityType::Goat => {
                        match state {
                            EntityState::Idle => {
                                let target_location = field.can_see_food(entity, &entity_q.to_readonly(), &tile_q);
                                if target_location.x != entity.target_location.x || target_location.y != entity.target_location.y {
                                    if !field.move_entity(&mut commands, &mut entity_q, &tile_q, entity, target_location) {
                                        simulating.simulating = false;
                                        println!("FAIL STATE");
                                        return;
                                    }
                                    simulating.simulating = true;
                                }
                            }
                            EntityState::Walking => {
                                if !field.move_entity(&mut commands, &mut entity_q, &tile_q, entity, entity.target_location) {
                                    simulating.simulating = false;
                                    println!("FAIL STATE");
                                    return;
                                }
                                simulating.simulating = true;
                            }
                            EntityState::Sliding => {
                                if entity.last_direction != MoveDirection::None {
                                    if !field.slide_entity(&mut commands, &mut entity_q, &tile_q, entity, entity.last_direction) {
                                        simulating.simulating = false;
                                        println!("FAIL STATE");
                                        return;
                                    }
                                    simulating.simulating = true;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if celebrate {
                for mut entity in &mut entity_q {
                    entity.state = EntityState::Celebrating;
                }
            }
        }
        if !simulating.simulating {
            println!("Simulation Over!");
            saving.saving = SaveStage::SaveUndo;
        }
    }
}