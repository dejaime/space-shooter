use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
    renderer::{
        resources::Tint,
    },
};

use crate::component::shield_component::Shield;

pub struct ShieldRecoverySystem;

impl<'s> System<'s> for ShieldRecoverySystem {
    type SystemData = (
        WriteStorage<'s, Tint>,
        WriteStorage<'s, Shield>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut tints, mut shields, time): Self::SystemData) {
        for tint in (&mut tints).join() {
            let time: f32 = time.absolute_time().as_secs_f32() as f32 * 4.0;

            let alpha = (0.5 + time.sin() * 0.51).max(0.0);
            
            tint.0.red = alpha;
            tint.0.green = alpha;
            tint.0.blue = alpha;
        }

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

            let current_shield_ratio = shield.hit_points / shield.max_hit_points;
        }
    }
}
