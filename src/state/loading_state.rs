use amethyst::prelude::*;

use amethyst::{
    core::transform::Transform, renderer::camera::Camera, GameData, SimpleState, StateData,
};

use crate::audio::initialise_music;
use crate::graphics::{initialise_graphics, SpritesHolder};
use crate::state::SpaceState;

#[derive(Default)]
pub struct LoadingState {}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_map = initialise_graphics(data.world);
        data.world.insert(SpritesHolder { sprite_map });

        initialise_music(data.world);

        initialise_camera(data.world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Push(Box::new(SpaceState {
            ..Default::default()
        }))
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = (1920.0, 1080.0);

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}
