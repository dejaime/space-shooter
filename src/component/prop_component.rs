use amethyst::ecs::prelude::{Component, NullStorage};

pub struct Prop {}

impl Component for Prop {
    type Storage = NullStorage<Self>;
}
