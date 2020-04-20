use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, WriteStorage},
};

use crate::component::weapon_type::*;

#[derive(SystemDesc)]
pub struct LaserFiringSystem;

impl<'s> System<'s> for LaserFiringSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Weapon>
    );

    fn run(&mut self, (entities, transforms, lasers): Self::SystemData) {
        //TODO: collision
    }
}
