use amethyst::{
    core::{
        math::{Vector3},
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
        for (prop_entity, prop, transform) in (&*entities, &props, &mut transforms).join() {
        for (entity, prop, transform) in (&*entities, &props, &mut transforms).join() {

            transform.prepend_translation(Vector3::new(prop.directional_speed.x, prop.directional_speed.y, 0.0));
            transform.prepend_rotation_z_axis(prop.rotational_speed * time.delta_seconds());
        }
    }
}
