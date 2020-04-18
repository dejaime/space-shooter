use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct LaserSystem;

use crate::component::laser_component::Laser;

impl<'s> System<'s> for LaserSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Laser>,
        Read<'s, Time>
    );

    fn run(&mut self, (entities, mut transforms, lasers, time): Self::SystemData) {
        for (laser_entity, transform, laser) in (&*entities, &mut transforms, &lasers).join() {
            //TODO: Move Lasers
        }
    }
}
