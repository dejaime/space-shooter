use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HealthType {
    Player,
    Enemy,
    Boss
}

pub struct Health {
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub lives: i32,
    pub time_since_last_hit: f32,
    pub health_type: HealthType,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
