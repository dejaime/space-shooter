use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::prop_component::Prop;
use crate::graphics::get_spritesheet_handle;
use rand::prelude::*;

pub fn spawn_prop(world: &mut World, rng: &mut ThreadRng) -> Entity {
    let is_asteroid = rng.gen_range(0.0, 1.0) > 0.9;
    let mut scale = rng.gen_range(0.1, 0.8);

    let (sprite_sheet_handle, sprite_number, directional_speed) = if is_asteroid {
        scale = rng.gen_range(0.3, 1.5);
        (
            get_spritesheet_handle(world, "asteroid"),
            rng.gen_range(0, 4) as usize,
            Vector2::new(rng.gen_range(-5.0, 5.0), -20.0 / scale * scale),
        )
    } else {
        (
            get_spritesheet_handle(world, "proton"),
            rng.gen_range(0, 3) as usize,
            Vector2::new(0.0, -1.0 * scale),
        )
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(rng.gen_range(-1000.0, 1000.0), 550.0, -0.1);
    if is_asteroid {
        transform.set_translation_z(-0.05);
    }
    transform.set_scale(Vector3::new(scale, scale, scale));

    let prop = Prop {
        directional_speed: directional_speed,
        rotational_speed: rng.gen_range(-3.0, 3.0),
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
