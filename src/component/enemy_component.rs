use amethyst::ecs::prelude::{Component, VecStorage};


#[derive(Default)]
pub struct Enemy {}

impl Component for Enemy {
    type Storage = VecStorage<Self>;
}
