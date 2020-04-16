use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    window::ScreenDimensions,
};

use crate::component::ship_component::Ship;

const SHIP_COLLISION_RADIUS: f32 = 32.0;
const SHIP_SPAWN_Y_OFFSET: f32 = 64.0;

pub fn spawn_player_ship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    //I need to use world.read_resource::<ScreenDimensions>() inside this short scope
    //  so I can use world.create_entity below (immutable (here) vs mutable borrow)
    let screen_dimensions = {
        let screen_dimensions_resource = world.read_resource::<ScreenDimensions>();
        (screen_dimensions_resource.width(), screen_dimensions_resource.height())
    };

    let mut local_transform = Transform::default();

    local_transform.set_translation(Vector3::new(
        screen_dimensions.0 / 2.0 - SHIP_COLLISION_RADIUS,
        SHIP_SPAWN_Y_OFFSET,
        0.0,
    ));

    world
        .create_entity()
        .with(Ship {
            radius: 0.0,
            velocity: 0.0,
        })
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: 0,
        })
        .build()
}
