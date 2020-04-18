use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct LaserMovementSystem;

use crate::component::laser_component::Laser;

const Y_MAX: f32 = 600.0;
const Y_MIN: f32 = -600.0;
const X_MAX: f32 = 1000.0;
const X_MIN: f32 = -1000.0;

impl<'s> System<'s> for LaserMovementSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Laser>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut transforms, mut lasers, time): Self::SystemData) {
        for (laser_entity, transform, laser) in (&*entities, &mut transforms, &mut lasers).join() {
            if transform.translation().x > X_MAX
                || transform.translation().x < X_MIN
                || transform.translation().y > Y_MAX
                || transform.translation().y < Y_MIN
            {
                let _ = entities.delete(laser_entity);
                continue;
            }

            let delta = time.delta_seconds();

            //Apply movement and rotation before modifying it with accelerations and torques
            transform.prepend_translation(laser.directional_speed * delta);
            transform.rotate_2d(laser.rotational_radian_speed * delta);
            laser.directional_speed += laser.directional_acceleration * delta;
            laser.directional_acceleration += laser.directional_torque * delta;
        }
    }
}
