use amethyst::ecs::prelude::{Component, VecStorage};

pub struct Enemy {}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}
