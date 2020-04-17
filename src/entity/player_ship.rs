use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::{SpriteRender},
    window::ScreenDimensions,
};

use crate::component::{ship_component::Ship, player_component::{Player, PlayerSeat}};
use crate::graphics::{get_spritesheet_handle};


const SHIP_COLLISION_RADIUS: f32 = 32.0;
const SHIP_SPAWN_Y_OFFSET: f32 = 0.0;

pub fn spawn_player_ship(world: &mut World, second_player: bool) -> Entity {
    //I need to use world.read_resource::<ScreenDimensions>() inside this short scope
    //  so I can use world.create_entity below (immutable (here) vs mutable borrow)
    let screen_dimensions = {
        let screen_dimensions_resource = world.read_resource::<ScreenDimensions>();
        (screen_dimensions_resource.width(), screen_dimensions_resource.height())
    };

    let (sprite_name, player_seat) = if second_player { ("player_2", PlayerSeat::P2) } else { ("player_1", PlayerSeat::P1) };
    

    let sprite_sheet_handle = get_spritesheet_handle(world, sprite_name);

    let mut local_transform = Transform::default();

    local_transform.set_translation(Vector3::new(
        screen_dimensions.0 / 2.0 - SHIP_COLLISION_RADIUS,
        SHIP_SPAWN_Y_OFFSET,
        0.0,
    ));

    world.register::<Ship>();
    world.register::<Player>();

    world
        .create_entity()
        .with(Player{seat: player_seat, ..Default::default()})
        .with(Ship {
            radius: 0.0,
            speed: 200.0,
        })
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        })
        .build()
}
