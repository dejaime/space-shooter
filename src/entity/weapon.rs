use amethyst::{
    core::{
        transform::{Parent, Transform},
    },
    ecs::prelude::{Entity, World},
    prelude::*,
};

use crate::component::weapon_component::Weapon;

pub fn spawn_weapon_entity(world: &mut World, weapon_component: Weapon, owner: &mut Entity) -> Entity {
    world
        .create_entity()
        .with(weapon_component)
        .with(Transform::default())
        .with(Parent::new(*owner))
        .build()
}
