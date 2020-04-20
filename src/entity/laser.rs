use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entities, LazyUpdate},
    renderer::{SpriteRender},
};

use crate::component::laser_component::MatchWeaponComponent;
use crate::component::laser_type::LaserType;
use crate::graphics::{SpritesHolder};

pub fn spawn_laser(
    entities: &Entities,
    lazy_update: &LazyUpdate,
    sprite_sheet_holder: &SpritesHolder,
    laser_type: LaserType,
    position: Vector3<f32>,
) {
    //let sprite_sheet_handle = get_spritesheet_handle(world, "proton");

    let mut local_transform = Transform::default();

    local_transform.set_translation(position);

    let weapon_component = MatchWeaponComponent(laser_type);

    let laser_entity = entities.create();

    lazy_update.insert(laser_entity, local_transform);
    lazy_update.insert(
        laser_entity,
        SpriteRender {
            sprite_sheet: sprite_sheet_holder
                .sprite_map
                .get("proton")
                .unwrap()
                .clone(),
            sprite_number: 0,
        },
    );
    lazy_update.insert(laser_entity, weapon_component);
}
