use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Component)]
#[storage(NullStorage)]
pub struct Enemy {}
