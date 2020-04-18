use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};
use crate::graphics::get_spritesheet_handle;

pub fn spawn_laser(world: &mut World, is_player: bool) -> Entity {
    
    let sprite_sheet_handle = get_spritesheet_handle(world, "proton");

    let mut local_transform = Transform::default();
    local_transform.set_translation(Vector3::new(0.0, 0.0, 0.0));

    world
        .create_entity()
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        })
        .build()
}
