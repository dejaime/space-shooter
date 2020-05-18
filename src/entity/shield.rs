use amethyst::{
    core::{
        transform::{Parent, Transform},
    },
    ecs::prelude::{Entity, World},
    prelude::*,
};

use crate::component::shield_component::Shield;

pub fn spawn_shield_entity(world: &mut World, shield_component: Shield, owner: &mut Entity) -> Entity {
    world
        .create_entity()
        .with(shield_component)
        .with(Transform::default())
        .with(Parent::new(*owner))
        .build()
}
