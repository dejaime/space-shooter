use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Enemy {}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}