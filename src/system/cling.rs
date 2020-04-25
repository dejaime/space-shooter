use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, ReadStorage, WriteStorage},
};

use crate::component::clingy_component::ClingTo;

pub struct ClingSystem;

impl<'a> System<'a> for ClingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ClingTo>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (entity, target, transform): Self::SystemData) {
        for (entity, t) in (&*entity, &target).join() {
            //TODO: Apply speed when perfect_follow is false instead or move instantly
            //          to whatever offset is configured
        }
    }
}
