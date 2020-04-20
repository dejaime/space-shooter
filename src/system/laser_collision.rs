use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, ReadStorage, System, SystemData},
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
        ReadStorage<'s, Ship>,
    );

    fn run(&mut self, (_entities, _transforms, _lasers, _ships): Self::SystemData) {
        //TODO: Collide
    }
}
