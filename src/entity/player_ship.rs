use amethyst::{
    assets::Handle,
    core::{timing::Time, transform::Transform},
    ecs::prelude::{Entity, World, WorldExt},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};


const SHIP_COLLISION_RADIUS: f32 = 64.0;
const SHIP_SPAWN_Y_OFFSET: f32 = 64.0;