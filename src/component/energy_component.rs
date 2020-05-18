use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Energy {
    pub energy: f32,
    pub max_energy: f32,
    pub energy_recovery_per_second: f32,
    pub recover_cooldown: f32,
    pub time_since_last_activation: f32,
}

impl Component for Energy {
    type Storage = DenseVecStorage<Self>;
}
