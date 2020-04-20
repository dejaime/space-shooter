use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entities, LazyUpdate},
    renderer::SpriteRender,
};

use crate::component::laser_component::laser_vector_from_weapon_type;
use crate::component::weapon_type::{WeaponType, PlayerWeapon};
use crate::graphics::SpritesHolder;

pub fn spawn_laser(
    entities: &Entities,
    lazy_update: &LazyUpdate,
    sprite_sheet_holder: &SpritesHolder,
    weapon_type: WeaponType,
    position: Vector3<f32>,
) {
    //let sprite_sheet_handle = get_spritesheet_handle(world, "proton");

    let laser_vector = laser_vector_from_weapon_type(weapon_type);

    for laser in laser_vector {
        let mut local_transform = Transform::default();

        local_transform.set_translation(position);

        let laser_entity = entities.create();

        let sheet_name = match weapon_type {
            WeaponType::Player(sub_type) => match sub_type {
                PlayerWeapon::Simple(_) => "laser",
                PlayerWeapon::Fast(_) => "minigun",
                _ => "proton",
            },
            _ => "proton",
        };
        
        let sprite_sheet = sprite_sheet_holder
            .sprite_map
            .get(sheet_name)
            .unwrap()
            .clone();

        lazy_update.insert(laser_entity, local_transform);
        lazy_update.insert(
            laser_entity,
            SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            },
        );
        lazy_update.insert(laser_entity, laser);
    }
}
