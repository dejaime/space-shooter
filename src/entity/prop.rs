use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::{
    player_component::{Player, PlayerSeat},
    ship_component::Ship,
};
use crate::graphics::get_spritesheet_handle;

pub fn spawn_prop(world: &mut World) -> Entity {
    let sprite_sheet_handle = get_spritesheet_handle(world, sprite_name);

    let mut local_transform = Transform::default();

    world
        .create_entity()
        .with(Prop {
            ..Default::default()
        })
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        })
        .build()
}
