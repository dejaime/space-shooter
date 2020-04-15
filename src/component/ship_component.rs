use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Ship {
    pub velocity: f32,
    pub radius:f32
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}