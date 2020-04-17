use amethyst::{
    core::{math::{Vector2, Vector3}, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::{
    prop_component::Prop,
};
use crate::graphics::get_spritesheet_handle;
use rand::prelude::*;


pub fn spawn_prop(world: &mut World, scale: f32) -> Entity {

    let mut rng = thread_rng();

    let is_asteroid = rand::random::<u8>() > 230;

    let (sprite_sheet_handle, sprite_number, directional_speed) = if is_asteroid {
        (
            get_spritesheet_handle(world, "asteroid"),
            (rand::random::<u8>() % 4) as usize,
            Vector2::new((2 - rand::random::<u8>() % 4) as f32, -1.0 * scale)
        )
    } else {
        (
            get_spritesheet_handle(world, "proton"),
            (rand::random::<u8>() % 3) as usize,
            Vector2::new(0.0, -1.0 * scale)
        )
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(rng.gen_range(-1000.0, 1000.0), 600.0, 10.0);
    transform.set_scale(Vector3::new(scale, scale, scale));

    let prop = Prop {
        directional_speed: directional_speed,
        rotational_speed: rng.gen_range(-10.0, 10.0)
    };

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_number,
    };

    world
    .create_entity()
    .with(prop)
    .with(transform)
    .with(sprite)
    .build()
}
