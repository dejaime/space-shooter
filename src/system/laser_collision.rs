use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, ReadStorage},
};

#[derive(SystemDesc)]
pub struct LaserCollisionSystem;

use crate::component::laser_component::Laser;
use crate::component::ship_component::Ship;

impl<'s> System<'s> for LaserCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Ship>
    );

    fn run(&mut self, (entities, transforms, lasers, ships): Self::SystemData) {
        //TODO: Collide
    }
}
