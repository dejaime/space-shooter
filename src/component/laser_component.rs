use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Component, VecStorage},
};

use super::weapon_type::{EnemyWeapon, PlayerWeapon, WeaponType};
use crate::component::player_component::PlayerSeat;

#[derive(Debug)]
pub struct Laser {
    pub weapon_type: WeaponType,
    pub owner_seat: PlayerSeat,
    pub directional_speed: Vector3<f32>,
    pub directional_acceleration: Vector3<f32>,
    pub directional_torque: Vector3<f32>,
    pub rotational_radian_speed: f32,
    pub damage: f32,
    pub radius: f32,
    pub destroy_on_hit: bool,
}

impl Component for Laser {
    type Storage = VecStorage<Self>;
}

pub fn laser_vector_from_weapon_type(weapon_type: WeaponType) -> Vec<Laser> {
    let default_laser = vec![Laser {
        owner_seat: PlayerSeat::NonPlayer,
        weapon_type: WeaponType::Enemy(EnemyWeapon::Simple),
        directional_speed: Vector3::new(0.0, 10.0, 0.0),
        directional_acceleration: Vector3::new(0.0, 0.1, 0.0),
        directional_torque: Vector3::new(0.0, 0.1, 0.0),
        rotational_radian_speed: 0.0,
        damage: 1.0,
        radius: 10.0,
        destroy_on_hit: true,
    }];

    let laser_vec = match weapon_type {
        WeaponType::Player(sub_type) => match sub_type {
            PlayerWeapon::Fast(seat) => vec![Laser {
                weapon_type: weapon_type,
                owner_seat: seat,
                directional_speed: Vector3::new(0.0, 1000.0, 0.0),
                directional_acceleration: Vector3::new(0.0, 1000.0, 0.0),
                directional_torque: Vector3::new(0.0, 0.0, 0.0),
                rotational_radian_speed: 0.0,
                damage: 1.0,
                radius: 10.0,
                destroy_on_hit: true,
            }],
            PlayerWeapon::Simple(seat) => vec![Laser {
                weapon_type: weapon_type,
                owner_seat: seat,
                directional_speed: Vector3::new(0.0, 400.0, 0.0),
                directional_acceleration: Vector3::new(0.0, 100.0, 0.0),
                directional_torque: Vector3::new(0.0, 100.0, 0.0),
                rotational_radian_speed: 0.0,
                damage: 1.0,
                radius: 10.0,
                destroy_on_hit: true,
            }],
            PlayerWeapon::Arc(seat) => vec![
                Laser {
                    weapon_type: weapon_type,
                    owner_seat: seat,
                    directional_speed: Vector3::new(400.0, -100.0, 0.0),
                    directional_acceleration: Vector3::new(-300.0, 1000.0, 0.0),
                    directional_torque: Vector3::new(0.0, 100.0, 0.0),
                    rotational_radian_speed: 0.0,
                    damage: 1.0,
                    radius: 10.0,
                    destroy_on_hit: true,
                },
                Laser {
                    weapon_type: weapon_type,
                    owner_seat: seat,
                    directional_speed: Vector3::new(-400.0, -100.0, 0.0),
                    directional_acceleration: Vector3::new(300.0, 1000.0, 0.0),
                    directional_torque: Vector3::new(0.0, 100.0, 0.0),
                    rotational_radian_speed: 0.0,
                    damage: 1.0,
                    radius: 10.0,
                    destroy_on_hit: true,
                },
            ],
            PlayerWeapon::V(seat) => vec![
                Laser {
                    weapon_type: weapon_type,
                    owner_seat: seat,
                    directional_speed: Vector3::new(300.0, 500.0, 0.0),
                    directional_acceleration: Vector3::new(0.0, 0.0, 0.0),
                    directional_torque: Vector3::new(0.0, 100.0, 0.0),
                    rotational_radian_speed: 0.0,
                    damage: 1.0,
                    radius: 10.0,
                    destroy_on_hit: true,
                },
                Laser {
                    weapon_type: weapon_type,
                    owner_seat: seat,
                    directional_speed: Vector3::new(-300.0, 500.0, 0.0),
                    directional_acceleration: Vector3::new(0.0, 0.0, 0.0),
                    directional_torque: Vector3::new(0.0, 100.0, 0.0),
                    rotational_radian_speed: 0.0,
                    damage: 1.0,
                    radius: 10.0,
                    destroy_on_hit: true,
                },
            ],
        },
        WeaponType::Enemy(sub_type) => match sub_type {
            Simple => default_laser,
            Fast => default_laser,
            Arc => default_laser,
            ZigZag => default_laser,
            BigSlow => default_laser,
        },
        WeaponType::Boss(sub_type) => match sub_type {
            Directional => default_laser,
            Homing => default_laser,
            DoubleSwipe => default_laser,
            Maze => default_laser,
            Horizontal => default_laser,
        },
    };

    laser_vec
}
