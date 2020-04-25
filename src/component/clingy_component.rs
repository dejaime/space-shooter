use amethyst::core::math::Vector3;

use amethyst::ecs::{Component, Entity, VecStorage};

pub struct ClingTo {
    target: Entity,
    offset: Vector3<f32>,
    max_distance: f32,
    speed: f32,
    perfect_follow: bool,
}

impl Component for ClingTo {
    type Storage = VecStorage<Self>;
}
