use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct BackgroundPropSystem;

use crate::component::Prop;

impl<'s> System<'s> for BackgroundPropSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Prop>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut transforms, props, time): Self::SystemData) {
        for (entity, prop, transform) in (&*entities, &props, &mut transforms).join() {
            
        }
    }
}
