use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Component, VecStorage, NullStorage},
};

use crate::component::player_component::PlayerSeat;


#[derive(Debug)]
pub struct Laser {
    pub directional_speed: Vector3<f32>,
    pub directional_acceleration: Vector3<f32>,
    pub directional_torque: Vector3<f32>,
    pub rotational_radian_speed: f32,
    pub radius: f32,
    pub destroy_on_hit: bool,
}

impl Component for Laser {
    type Storage = VecStorage<Self>;
}


#[derive(Debug, Default)]
pub struct EnemyLaser {}

impl Component for EnemyLaser {
    type Storage = NullStorage<Self>;
}


#[derive(Debug)]
pub struct PlayerLaser {
    pub seat: PlayerSeat,
}

impl Component for PlayerLaser {
    type Storage = VecStorage<Self>;
}
