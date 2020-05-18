use amethyst::ecs::prelude::{Component, VecStorage};

use crate::component::player_component::PlayerSeat;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Shield {
    pub owner_seat: PlayerSeat,
    pub hit_points: f32,
    pub max_hit_points: f32,
    pub recovery_cooldown: f32,
    pub time_since_last_hit: f32,
    pub time_since_last_break: f32,
    pub hit_point_recovery_per_second: f32,
    pub fully_recovered_after_break: bool,
}

impl Component for Shield {
    type Storage = VecStorage<Self>;
}

impl Default for Shield {
    fn default() -> Shield {
        owner_seat: PlayerSeat::NonPlayer,
        hit_points: 100.0,
        max_hit_points: 100.0,
        recovery_cooldown: 0.0,
        time_since_last_hit: 10.0,
        time_since_last_break: 10.0,
        hit_point_recovery_per_second: 0.0,
        fully_recovered_after_break: true,
    }
}
