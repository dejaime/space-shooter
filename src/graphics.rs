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

fn load_spritesheet(world: &World, image_file: &str, definition_file: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let texture_handle = load_texture(world, image_file);
    loader.load(
        definition_file,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn initialise_graphics(world: &mut World) {

}
