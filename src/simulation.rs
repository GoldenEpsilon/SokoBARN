use crate::*;
use crate::game::*;

use bevy::prelude::*;

pub fn simulate(
    mut commands: Commands, 
    mut field: ResMut<Field>,
    time: Res<Time>,
    mut simulating: ResMut<SimulateRes>,
    mut entity_q: Query<&mut GameEntity>,
    tile_q: Query<&Tile>,){
    if simulating.simulating {
        field.simulate_timer.tick(time.delta());
        if field.simulate_timer.just_finished() {
            println!("Simulation Tick!");
            simulating.simulating = false;
            for entity in field.get_entities(&entity_q.to_readonly()) {
                match entity.entity_type {
                    EntityType::Chicken | EntityType::Pig | EntityType::Horse | EntityType::Goat => {
                        match entity.state {
                            EntityState::Idle | EntityState::Walking => {
                                let move_dir = field.can_see_food(entity, &entity_q.to_readonly(), &tile_q);
                                field.move_entity(&mut commands, &mut entity_q, &tile_q, entity, move_dir);
                                simulating.simulating = true;
                            }
                            EntityState::Sliding => {
                                field.move_entity(&mut commands, &mut entity_q, &tile_q, entity, entity.last_direction);
                                simulating.simulating = true;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        if !simulating.simulating {
            println!("Simulation Over!");
        }
    }
}