use amethyst::{
    core::timing::Time,
    ecs::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::component::{
    enemy_component::Enemy, health_component::Health, player_component::Player,
};

pub struct HealthSystem;

impl<'s> System<'s> for HealthSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut healths, players, enemies, time): Self::SystemData) {
        for (player_entity, health, _player) in (&*entities, &mut healths, &players).join() {
            health.time_since_last_hit += time.delta_seconds();
            if health.health <= 0.0 {
                //TODO: deal with game over
                let _ = entities.delete(player_entity);
            }
            continue;
        }

        for (enemy_entity, health, _enemy) in (&*entities, &mut healths, &enemies).join() {
            health.time_since_last_hit += time.delta_seconds();
            if health.health <= 0.0 {
                let _ = entities.delete(enemy_entity);
            }
            continue;
        }
    }
}
