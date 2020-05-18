use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
};

use crate::component::shield_component::Shield;

pub struct ShieldRecoverySystem;

impl<'s> System<'s> for ShieldRecoverySystem {
    type SystemData = (WriteStorage<'s, Shield>, Read<'s, Time>);

    fn run(&mut self, (mut shields, time): Self::SystemData) {
        for shield in (&mut shields).join() {
            if shield.time_since_last_hit < shield.recovery_cooldown {
                continue;
            }

            //if shield was broken, double recovery cooldown
            if shield.time_since_last_break < shield.recovery_cooldown * 2.0 {
                continue;
            }

            //if shield was broken, recover 3x faster (adding 2x here)
            if shield.fully_recovered_after_break {
                shield.hit_points =
                    shield.hit_point_recovery_per_second * time.delta_seconds() * 3.0;
            } else {
                shield.hit_points = shield.hit_point_recovery_per_second * time.delta_seconds();
            }

            if shield.hit_points > shield.max_hit_points {
                shield.hit_points = shield.max_hit_points;
            }
        }
    }
}
