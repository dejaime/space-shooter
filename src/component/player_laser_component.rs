use amethyst::{
    ecs::prelude::{Component, VecStorage},
    core::math::Vector2,
};

use crate::component::player_component::PlayerSeat;

#[derive(Debug)]
pub struct PlayerLaser {
    pub directional_speed: Vector2<f32>,
    pub directional_acceleration: Vector2<f32>,
    pub directional_torque: Vector2<f32>,
    pub seat: PlayerSeat,
    pub radius: f32,
    pub destroy_on_hit: bool,
}

impl Component for PlayerLaser {
    type Storage = VecStorage<Self>;
}
