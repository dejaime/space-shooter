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
            //For every Laser
            for (laser_entity, laser_transform, laser) in (&*entities, &transforms, &lasers).join()
            {
                if laser.owner_seat == PlayerSeat::NonPlayer
                    && ship_health.health_type == HealthType::Player
                {
                    if is_colliding(ship_transform, ship, laser_transform, laser) {
                        //HIT

                        ship_health.health -= laser.damage;
                        ship_health.time_since_last_hit = 0.0;

                        if laser.destroy_on_hit {
                            let _ = entities.delete(laser_entity);
                        }
                    }
                } else if laser.owner_seat != PlayerSeat::NonPlayer
                    && ship_health.health_type != HealthType::Player
                {
                    if is_colliding(ship_transform, ship, laser_transform, laser) {
                        //HIT

                        if laser.destroy_on_hit {
                            let _ = entities.delete(laser_entity);
                        }
                    }
                }
            }
        }
    }
}

fn is_colliding(
    ship_transform: &Transform,
    ship_component: &Ship,
    laser_transform: &Transform,
    laser_component: &Laser,
) -> bool {
    let ship_translation = ship_transform.translation();
    let laser_translation = laser_transform.translation();

    let current_distance_squared = (
        (ship_translation.x - laser_translation.x) * (ship_translation.x - laser_translation.x),
        (ship_translation.y - laser_translation.y) * (ship_translation.y - laser_translation.y),
    );

    let current_distance_squared: f32 = current_distance_squared.0 + current_distance_squared.1;

    let min_collision_distance_squared = ship_component.radius * ship_component.radius
        + laser_component.radius * laser_component.radius;
    current_distance_squared < min_collision_distance_squared
}
