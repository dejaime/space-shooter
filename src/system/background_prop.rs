use amethyst::{
    core::{math::Vector3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Write, Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::state::space_state::PropCounter;

#[derive(SystemDesc)]
pub struct BackgroundPropSystem;

use crate::component::Prop;

impl<'s> System<'s> for BackgroundPropSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Prop>,
        Read<'s, Time>,
        Write<'s, PropCounter>,
    );

    fn run(&mut self, (entities, mut transforms, props, time, mut prop_counter): Self::SystemData) {
        for (prop_entity, prop, transform) in (&*entities, &props, &mut transforms).join() {
            if transform.translation().y < -400.0 {
                let _ = entities.delete(prop_entity);
                prop_counter.active_props_count -= 1;
                continue;
            }

            transform.prepend_translation(Vector3::new(
                prop.directional_speed.x,
                prop.directional_speed.y,
                0.0,
            ));
            transform.prepend_rotation_z_axis(prop.rotational_speed * time.delta_seconds());
        }
    }
}
