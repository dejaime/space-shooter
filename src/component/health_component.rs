use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct Health {
    pub speed: f32,
    pub radius: f32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
