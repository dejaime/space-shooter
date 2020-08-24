use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};

use crate::graphics::get_spritesheet_handle;
use rand::prelude::*;


const BG_SCALE: f32 = 100.0;

pub fn spawn_background(world: &mut World) -> Entity {

    let sprite_sheet_handle = get_spritesheet_handle(world, "black_bg");

    let mut transform = Transform::default();
    transform.set_translation_x(-1.0);
    transform.set_translation_y(-1.0);
    transform.set_scale(Vector3::new(BG_SCALE, BG_SCALE, BG_SCALE));

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .build()
}
