use amethyst::prelude::*;

use amethyst::{
    assets::Handle, core::transform::Transform, renderer::camera::Camera, renderer::SpriteSheet,
    window::ScreenDimensions, GameData, SimpleState, StateData,
};

use std::collections::BTreeMap;

use crate::audio::initialise_music;
use crate::graphics::initialise_graphics;

#[derive(Default)]
pub struct LoadingState {}


pub struct SpritesHolder {
    pub sprite_map: BTreeMap<&'static str, Handle<SpriteSheet>>,
}


impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        
        let sprite_map = initialise_graphics(data.world);
        data.world.insert(SpritesHolder {sprite_map});

        initialise_music(data.world);

        initialise_camera(data.world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
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
