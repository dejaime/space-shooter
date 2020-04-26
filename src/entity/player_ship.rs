use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::{
    player_component::{Player, PlayerSeat},
    ship_component::Ship,
    weapon_component::Weapon,
    weapon_type::*,
};
use crate::graphics::get_spritesheet_handle;
use crate::entity::weapon::spawn_weapon_entity;

const _SHIP_COLLISION_RADIUS: f32 = 32.0;
const SHIP_SPEED: f32 = 400.0;

pub fn spawn_player_ship(world: &mut World, second_player: bool) -> Entity {
    let (sprite_name, player_seat) = if second_player {
        ("player_2", PlayerSeat::P2)
    } else {
        ("player_1", PlayerSeat::P1)
    };

    let sprite_sheet_handle = get_spritesheet_handle(world, sprite_name);

    let mut local_transform = Transform::default();

    let spawn_y = if second_player { 128.0 } else { 0.0 };
    local_transform.set_translation(Vector3::new(0.0, spawn_y, 0.0));

    let weapon_component = if second_player {
        Weapon {
            owner_seat: player_seat,
            weapon_type: WeaponType::Player(PlayerWeapon::Fast(player_seat)),
            next_shot_time: 0.0,
            cooldown: 0.05,
        }
    } else {
        Weapon {
            owner_seat: player_seat,
            weapon_type: WeaponType::Player(PlayerWeapon::Simple(player_seat)),
            next_shot_time: 0.0,
            cooldown: 0.05,
        }
    };

    let mut ship = world
        .create_entity()
        .with(Player {
            seat: player_seat,
            ..Default::default()
        })
        .with(Ship {
            radius: 0.0,
            speed: SHIP_SPEED,
        })
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        })
        .build();
    
    spawn_weapon_entity(world, weapon_component, &mut ship);

    ship
}
