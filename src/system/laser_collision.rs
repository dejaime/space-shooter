use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct LaserCollisionSystem;

use crate::component::{
    health_component::{Health, HealthType},
    laser_component::Laser,
    player_component::PlayerSeat,
    ship_component::Ship,
};

impl<'s> System<'s> for LaserCollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Health>,
    );

    fn run(&mut self, (entities, transforms, lasers, ships, mut healths): Self::SystemData) {
        //For every ship
        for (ship_transform, ship, ship_health) in (&transforms, &ships, &mut healths).join() {
            let ship_translation = ship_transform.translation();
            let ship_radius_squared = ship.radius * ship.radius;
            //For every Laser
            for (laser_entity, laser_transform, laser) in (&*entities, &transforms, &lasers).join() {
                if laser.owner_seat == PlayerSeat::NonPlayer
                    && ship_health.health_type == HealthType::Player {
                    let laser_translation = laser_transform.translation();

                    let x_distance_squared = (laser_translation.x - ship_translation.x) * (laser_translation.x - ship_translation.x);
                    let y_distance_squared = (laser_translation.y - ship_translation.y) * (laser_translation.y - ship_translation.y);
                    let distance_squared = x_distance_squared + y_distance_squared;

                    if distance_squared < ship_radius_squared {
                        // if laser.destroy_on_hit {
                            let _ = entities.delete(laser_entity);
                        // }
                    }
                } else {
                }
            }
        }
    }
}
