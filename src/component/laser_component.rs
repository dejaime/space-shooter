use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Component, VecStorage, NullStorage},
};

use crate::component::player_component::PlayerSeat;
use crate::weapon::*;

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

pub fn MatchWeaponComponent (weapon_type: WeaponType) -> Laser {
    
    let default_laser = Laser {
        directional_speed: Vector3::new(0.0, -100.0, 0.0),
        directional_acceleration: Vector3::new(0.0, -100.0, 0.0),
        directional_torque: Vector3::new(0.0, -100.0, 0.0),
        rotational_radian_speed: 0.0,
        radius: 0.0,
        destroy_on_hit: true,
    };

    default_laser
}