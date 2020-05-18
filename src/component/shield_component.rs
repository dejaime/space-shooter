use amethyst::{
    ecs::prelude::{Component, VecStorage},
};

use crate::component::{player_component::PlayerSeat};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Shield {
    pub owner_seat: PlayerSeat,
    pub hit_points: f32,
    pub max_hit_points: f32,
    pub recovery_cooldown: f32,
    pub time_since_last_hit: f32,
    pub time_since_last_break: f32,
    pub hit_point_recovery_per_second: f32,
}

impl Component for Shield {
    type Storage = VecStorage<Self>;
}
