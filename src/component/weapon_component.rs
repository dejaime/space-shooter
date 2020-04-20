use amethyst::{
    core::math::Vector3,
    ecs::prelude::{Component, NullStorage, VecStorage},
};

#[derive(Debug)]
pub struct Weapon {
    pub type: WeaponType,
    pub cooldown: f32,
}

impl Component for Weapon {
    type Storage = VecStorage<Self>;
}
