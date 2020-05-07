use amethyst::{
    core::{math::Vector3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{
        Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage,
    },
};

use crate::component::{player_component::PlayerSeat, weapon_component::Weapon};
use crate::entity::laser::spawn_laser;
use crate::graphics::SpritesHolder;

#[derive(SystemDesc)]
pub struct LaserFiringSystem;

impl<'s> System<'s> for LaserFiringSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Weapon>,
        Read<'s, Time>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, SpritesHolder>,
        Entities<'s>,
    );

    fn run(
        &mut self,
        (transforms, mut weapons, time, lazy_update, sprite_sheet_holder, entities): Self::SystemData,
    ) {
        for (transform, weapon) in (&transforms, &mut weapons).join() {
            if weapon.next_shot_time <= 0.0 {
                weapon.next_shot_time += weapon.cooldown;

                let y_offset = {
                    if weapon.owner_seat == PlayerSeat::NonPlayer {
                        -32.0
                    } else {
                        32.0
                    }
                };

                let spawn_point = Vector3::new(
                    transform.global_matrix()[12],
                    transform.global_matrix()[13] + y_offset,
                    0.01,
                );
                spawn_laser(
                    &entities,
                    &lazy_update,
                    &sprite_sheet_holder,
                    weapon.weapon_type,
                    spawn_point,
                );
            } else {
                weapon.next_shot_time -= time.delta_seconds();
            }
        }
    }
}
