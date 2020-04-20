use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entities, LazyUpdate},
    renderer::{SpriteRender},
};

use crate::component::laser_component::LaserVectorFromWeaponType;
use crate::component::weapon_type::WeaponType;
use crate::graphics::{SpritesHolder};

pub fn spawn_laser(
    entities: &Entities,
    lazy_update: &LazyUpdate,
    sprite_sheet_holder: &SpritesHolder,
    weapon_type: WeaponType,
    position: Vector3<f32>,
) {
    //let sprite_sheet_handle = get_spritesheet_handle(world, "proton");

    let laser_vector = LaserVectorFromWeaponType(weapon_type);

    for laser in laser_vector {
        let mut local_transform = Transform::default();

        local_transform.set_translation(position);

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
        lazy_update.insert(laser_entity, laser);
    }


}
