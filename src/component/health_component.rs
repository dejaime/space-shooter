use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct EnemyHealth {
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub last_hit_time: f32,
}

impl Component for EnemyHealth {
    type Storage = DenseVecStorage<Self>;
}

pub struct PlayerHealth {
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub lives: i32,
    pub last_hit_time: f32,
}

impl Component for PlayerHealth {
    type Storage = DenseVecStorage<Self>;
}

pub struct BossHealth {
    pub health: f32,
    pub max_health: f32,
    pub last_hit_time: f32,
}

impl Component for BossHealth {
    type Storage = DenseVecStorage<Self>;
}
