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



pub fn initialise_graphics(world: &mut World) -> HashMap<&str, Handle<SpriteSheet>>{
    let mut spritesheet_hashmap = HashMap::new();

    //ASTEROID
    spritesheet_hashmap.insert("asteroid", load_spritesheet(world, "texture/asteroid.png", "texture/asteroid_sheet.ron"));

    //LASER
    spritesheet_hashmap.insert("laser", load_spritesheet(world, "texture/laser.png", "texture/laser_sheet.ron"));

    //MINIGUN
    spritesheet_hashmap.insert("minigun", load_spritesheet(world, "texture/minigun.png", "texture/minigun_sheet.ron"));

    //PLASMA
    spritesheet_hashmap.insert("plasma", load_spritesheet(world, "texture/plasma.png", "texture/plasma_sheet.ron"));

    //PROTON
    spritesheet_hashmap.insert("proton", load_spritesheet(world, "texture/proton.png", "texture/proton_sheet.ron"));
    
    //EXHAUST
    spritesheet_hashmap.insert("exhaust", load_spritesheet(world, "texture/exhaust.png", "texture/exhaust_sheet.ron"));

    //POWER UPS
    spritesheet_hashmap.insert("powerup", load_spritesheet(world, "texture/powerup.png", "texture/powerup_sheet.ron"));

    //EXPLOSIONS
    spritesheet_hashmap.insert("explosion_1", load_spritesheet(world, "texture/explosion.png", "texture/explosion_1_sheet.ron"));
    spritesheet_hashmap.insert("explosion_2", load_spritesheet(world, "texture/explosion.png", "texture/explosion_2_sheet.ron"));

    //PLAYERS
    spritesheet_hashmap.insert("player_1", load_spritesheet(world, "texture/player.png", "texture/player_1_sheet.ron"));
    spritesheet_hashmap.insert("player_2", load_spritesheet(world, "texture/player.png", "texture/player_2_sheet.ron"));

    //ENEMIES
    spritesheet_hashmap.insert("enemy_1", load_spritesheet(world, "texture/enemy.png", "texture/enemy_1_sheet.ron"));
    spritesheet_hashmap.insert("enemy_2", load_spritesheet(world, "texture/enemy.png", "texture/enemy_2_sheet.ron"));
    spritesheet_hashmap.insert("enemy_3", load_spritesheet(world, "texture/enemy.png", "texture/enemy_3_sheet.ron"));
    spritesheet_hashmap.insert("enemy_4", load_spritesheet(world, "texture/enemy.png", "texture/enemy_4_sheet.ron"));
    spritesheet_hashmap.insert("enemy_5", load_spritesheet(world, "texture/enemy.png", "texture/enemy_5_sheet.ron"));
    spritesheet_hashmap.insert("enemy_6", load_spritesheet(world, "texture/enemy.png", "texture/enemy_6_sheet.ron"));
    
    spritesheet_hashmap

}
