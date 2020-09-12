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
    pub lives: i32,
    pub time_since_last_hit: f32,
    pub health_type: HealthType,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Health {
    fn default() -> Self {
        Health {
            health: 100.0,
            max_health: 100.0,
            lives: 0,
            time_since_last_hit: 0.0,
            health_type: HealthType::Enemy,
        }
    }
}
