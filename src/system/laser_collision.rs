use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct LaserMovementSystem;

use crate::component::laser_component::Laser;
use crate::component::player_laser_component::PlayerLaser;
use crate::component::enemy_laser_component::EnemyLaser;

impl<'s> System<'s> for LaserMovementSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Laser>,
        ReadStorage<'s, PlayerLaser>,
        ReadStorage<'s, EnemyLaser>,
    );

    fn run(&mut self, (entities, transforms, lasers): Self::SystemData) {
        //TODO: collision
    }
}
