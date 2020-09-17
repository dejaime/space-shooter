use amethyst::prelude::*;

use amethyst::{
    core::transform::Transform, renderer::camera::Camera, GameData, SimpleState, StateData,
};

use crate::audio::initialise_music;
use crate::graphics::{initialise_graphics, SpritesHolder};
use crate::state::{MenuState};
use crate::fonts::{FontHolder, initialise_fonts};

#[derive(Default)]
pub struct LoadingState {}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let sprite_map = initialise_graphics(data.world);
        data.world.insert(SpritesHolder { sprite_map });

        initialise_music(data.world);

        initialise_camera(data.world);

        let font_handle = initialise_fonts(data.world);
        data.world.insert(FontHolder{font_handle});
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Push(Box::new(MenuState {
            ..Default::default()
        }))
    }
}

fn initialise_camera(world: &mut World) {
    let (width, height) = (1920.0, 1080.0);

    let mut camera_transform = Transform::default();
    //This makes it so Max Z for entities is <1001. Change as necessary.
    camera_transform.set_translation_z(1001.0);

    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();
}
