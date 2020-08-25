use amethyst::{
    core::{transform::{Parent, Transform}, math::Vector3},
    ecs::prelude::{Entity, World},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint, SpriteRender, Transparent},
};

use crate::graphics::SpritesHolder;

use crate::component::shield_component::Shield;

pub fn spawn_shield_entity(
    world: &mut World,
    shield_component: Shield,
    owner: &mut Entity,
) -> Entity {
    let sprite_sheet;
    {
        let sprite_sheet_holder = &*(world.try_fetch::<SpritesHolder>().unwrap());

        sprite_sheet = sprite_sheet_holder
            .sprite_map
            .get("explosion_1")
            .unwrap()
            .clone();
    }

    let mut local_transform = Transform::default();
    local_transform.set_translation(Vector3::new(0.0, 0.0, 0.1));

    world
        .create_entity()
        .with(shield_component)
        .with(local_transform)
        .with(SpriteRender {
            sprite_sheet,
            sprite_number: 8,
        })
        .with(Tint(Srgba::new(1.0, 1.0, 1.0, 0.01)))
        .with(Transparent {})
        .with(Parent::new(*owner))
        .build()
}
