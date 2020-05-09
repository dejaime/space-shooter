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
    pub last_hit_time: f32,
    pub health_type: HealthType,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
