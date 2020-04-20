use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Component, NullStorage, VecStorage},
};

use super::laser_type::{LaserType, PlayerLaser};
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

pub fn MatchWeaponComponent(weapon_type: LaserType) -> Laser {
    let default_laser = Laser {
        directional_speed: Vector3::new(0.0, 10.0, 0.0),
        directional_acceleration: Vector3::new(0.0, 0.1, 0.0),
        directional_torque: Vector3::new(0.0, 0.1, 0.0),
        rotational_radian_speed: 0.0,
        radius: 10.0,
        destroy_on_hit: true,
    };

    let component = match weapon_type {
        LaserType::Player(sub_type) => match sub_type {
            Simple => Laser {
                directional_speed: Vector3::new(0.0, 10.0, 0.0),
                directional_acceleration: Vector3::new(0.0, 0.1, 0.0),
                directional_torque: Vector3::new(0.0, 0.1, 0.0),
                rotational_radian_speed: 0.0,
                radius: 10.0,
                destroy_on_hit: true,
            },
            Fast => default_laser,
            Arc => default_laser,
        },
        LaserType::Enemy(sub_type) => match sub_type {
            Simple => default_laser,
            Fast => default_laser,
            Arc => default_laser,
            ZigZag => default_laser,
            BigSlow => default_laser,
        },
        LaserType::Boss(sub_type) => match sub_type {
            Directional => default_laser,
            Homing => default_laser,
            DoubleSwipe => default_laser,
            Maze => default_laser,
            Horizontal => default_laser,
        },
    };

    component
}
