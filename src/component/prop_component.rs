use amethyst::{ecs::prelude::{Component, VecStorage}, core::math::Vector2};

pub struct Prop {
    pub directional_speed: Vector2<f32>,
    pub rotational_speed: f32
}

impl Component for Prop {
    type Storage = VecStorage<Self>;
}
