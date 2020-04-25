use amethyst::core::math::Vector3;

use amethyst::ecs::{Component, Entity, VecStorage};

pub struct ClingTo {
    pub target: Entity,
    pub offset: Vector3<f32>,
    pub max_distance: f32,
    pub speed: f32,
    pub perfect_follow: bool,
}

impl Component for ClingTo {
    type Storage = VecStorage<Self>;
}
