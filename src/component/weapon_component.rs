use amethyst::{
    ecs::prelude::{Component, VecStorage},
};

use crate::component::weapon_type::WeaponType;

#[derive(Debug)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub next_shot_time: f32,
    pub cooldown: f32,
}

impl Component for Weapon {
    type Storage = VecStorage<Self>;
}
