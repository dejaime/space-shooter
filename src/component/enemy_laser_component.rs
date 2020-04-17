use amethyst::{
    ecs::prelude::{Component, NullStorage},
};

#[derive(Debug)]
pub struct EnemyLaser {}

impl Component for EnemyLaser {
    type Storage = NullStorage<Self>;
}
