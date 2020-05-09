use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::component::{
    enemy_component::Enemy,
    player_component::{Player, PlayerSeat},
    health_component::Health,
};

pub struct HealthSystem;

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Time>
    );

    fn run(&mut self, (entities, mut healths, players, enemies, time): Self::SystemData) {
        for (player_entity, health, player) in (&*entities, &mut healths, &players).join() {
            health.time_since_last_hit += time.delta_seconds();
            if health.health <= 0.0 {
                //TODO: deal with game over
                let _ = entities.delete(player_entity);
            }
            continue;
        }

        for (enemy_entity, health, enemy) in (&*entities, &mut healths, &enemies).join() {
            health.time_since_last_hit += time.delta_seconds();
            if health.health <= 0.0 {
                let _ = entities.delete(enemy_entity);
            }
            continue;
        }
    }
}
