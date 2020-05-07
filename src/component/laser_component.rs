use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Component, VecStorage},
};

use super::weapon_type::{BossWeapon, EnemyWeapon, PlayerWeapon, WeaponType};
use crate::component::player_component::PlayerSeat;
use rand::prelude::*;

#[derive(Debug)]
pub struct Laser {
    pub weapon_type: WeaponType,
    pub owner_seat: PlayerSeat,
    pub initial_point: Vector3<f32>,
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

pub fn laser_vector_from_weapon_type(
    weapon_type: WeaponType,
    emit_point: Vector3<f32>,
) -> Vec<Laser> {
    let default_laser = vec![Laser {
        owner_seat: PlayerSeat::NonPlayer,
        weapon_type: WeaponType::Enemy(EnemyWeapon::Simple),
        initial_point: emit_point,
        directional_speed: Vector3::new(0.0, 10.0, 0.0),
        directional_acceleration: Vector3::new(0.0, 0.1, 0.0),
        directional_torque: Vector3::new(0.0, 0.1, 0.0),
        rotational_radian_speed: 0.0,
        damage: 1.0,
        radius: 10.0,
        destroy_on_hit: true,
    }];

    let mut rng = thread_rng();
    let x_dir = rng.gen_range(-1.0, 1.0);

    let laser_vec = match weapon_type {
        WeaponType::Player(sub_type) => match sub_type {
            PlayerWeapon::Fast(seat) => vec![Laser {
                weapon_type: weapon_type,
                initial_point: emit_point,
                owner_seat: seat,
                directional_speed: Vector3::new(0.0, 1000.0, 0.0),
                directional_acceleration: Vector3::new(0.0, 1000.0, 0.0),
                directional_torque: Vector3::new(0.0, 0.0, 0.0),
                rotational_radian_speed: 0.0,
                damage: 1.0,
                radius: 10.0,
                destroy_on_hit: true,
            }],
            PlayerWeapon::Simple(seat) => vec![
                Laser {
                    weapon_type: weapon_type,
                    initial_point: emit_point + Vector3::new(-10.0, 0.0, 0.0),
                    owner_seat: seat,
                    directional_speed: Vector3::new(0.0, 400.0, 0.0),
                    directional_acceleration: Vector3::new(0.0, 100.0, 0.0),
                    directional_torque: Vector3::new(0.0, 100.0, 0.0),
                    rotational_radian_speed: 0.0,
                    damage: 0.6,
                    radius: 10.0,
                    destroy_on_hit: true,
                },
                Laser {
                    weapon_type: weapon_type,
                    initial_point: emit_point + Vector3::new(10.0, 0.0, 0.0),
                    owner_seat: seat,
                    directional_speed: Vector3::new(0.0, 400.0, 0.0),
                    directional_acceleration: Vector3::new(0.0, 100.0, 0.0),
                    directional_torque: Vector3::new(0.0, 100.0, 0.0),
                    rotational_radian_speed: 0.0,
                    damage: 0.6,
                    radius: 10.0,
                    destroy_on_hit: true,
                },
            ],
            PlayerWeapon::Arc(seat) => vec![
                Laser {
                    weapon_type: weapon_type,
                    initial_point: emit_point + Vector3::new(-10.0, 0.0, 0.0),
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
                    initial_point: emit_point + Vector3::new(10.0, 0.0, 0.0),
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
                    initial_point: emit_point + Vector3::new(-10.0, 0.0, 0.0),
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
                    initial_point: emit_point + Vector3::new(10.0, 0.0, 0.0),
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
            EnemyWeapon::Simple => vec![Laser {
                weapon_type: weapon_type,
                initial_point: emit_point,
                owner_seat: PlayerSeat::NonPlayer,
                directional_speed: Vector3::new(x_dir * 100.0, -150.0, 0.0),
                directional_acceleration: Vector3::new(x_dir * 50.0, -50.0, 0.0),
                directional_torque: Vector3::new(0.0, 0.0, 0.0),
                rotational_radian_speed: 0.0,
                damage: 1.0,
                radius: 10.0,
                destroy_on_hit: true,
            }],
            EnemyWeapon::Fast => default_laser,
            EnemyWeapon::Arc => default_laser,
            EnemyWeapon::ZigZag => default_laser,
            EnemyWeapon::BigSlow => default_laser,
        },
        WeaponType::Boss(sub_type) => match sub_type {
            BossWeapon::Directional => default_laser,
            BossWeapon::Homing => default_laser,
            BossWeapon::DoubleSwipe => default_laser,
            BossWeapon::Maze => default_laser,
            BossWeapon::Horizontal => default_laser,
        },
    };

    laser_vec
}
