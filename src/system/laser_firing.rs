use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::component::weapon_type::*;
use crate::component::weapon_component::Weapon;

#[derive(SystemDesc)]
pub struct LaserFiringSystem;

impl<'s> System<'s> for LaserFiringSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Weapon>,
        Read<'s, Time>
    );

    fn run(&mut self, (_entities, _transforms, mut _weapons, _time): Self::SystemData) {
        //TODO: collision
    }
}
