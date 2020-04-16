use amethyst::prelude::*;

use amethyst::{
    core::transform::Transform, renderer::camera::Camera, window::ScreenDimensions, GameData,
    SimpleState, StateData,
};

use crate::audio::initialise_music;
use crate::graphics::initialise_graphics;

#[derive(Default)]
pub struct LoadingState {

}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_music(data.world);
        initialise_graphics(data.world);
        initialise_camera(data.world);
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = {
        let screen_dimensions = world.read_resource::<ScreenDimensions>();
        (screen_dimensions.width(), screen_dimensions.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}
