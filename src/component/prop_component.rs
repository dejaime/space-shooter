use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, VecStorage},
};

pub struct Prop {
    pub directional_speed: Vector2<f32>,
    pub rotational_speed: f32,
}

impl Component for Prop {
    type Storage = VecStorage<Self>;
}
