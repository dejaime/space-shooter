use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::{
    enemy_component::Enemy, player_component::PlayerSeat, ship_component::Ship,
    weapon_component::Weapon, weapon_type::*,
    health_component::{Health, HealthType},
};
use crate::entity::weapon::spawn_weapon_entity;
use crate::graphics::get_spritesheet_handle;

const _SHIP_COLLISION_RADIUS: f32 = 32.0;
const SHIP_SPEED: f32 = 400.0;

pub fn spawn_simple_enemy(world: &mut World) -> Entity {
    let sprite_sheet_handle = get_spritesheet_handle(world, "enemy_1");

    let mut local_transform = Transform::default();

    let spawn_y = 256.0;
    local_transform.set_translation(Vector3::new(0.0, spawn_y, 0.0));

    let weapon_component = Weapon {
        owner_seat: PlayerSeat::NonPlayer,
        weapon_type: WeaponType::Enemy(EnemyWeapon::Simple),
        next_shot_time: 0.0,
        cooldown: 0.5,
    };

    let mut ship = world
        .create_entity()
        .with(Enemy {})
        .with(Health {
            health: 0.0,
            max_health: 0.0,
            shield: 0.0,
            lives: 1,
            last_hit_time: 0.0,
            health_type: HealthType::Enemy,
        })
        .with(Ship {
            radius: 30.0,
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
