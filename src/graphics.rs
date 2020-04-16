use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use std::collections::HashMap;

#[derive(Clone)]
pub struct TextureResource {
    //pub texture_map: HashMap<str, Texture>
}

fn load_texture(world: &World, image_file: &str) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(image_file, ImageFormat::default(), (), &texture_storage)
}

pub fn initialise_graphics(world: &mut World) {

fn load_sprite_sheets(world: &mut World) {

}
