use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use std::collections::BTreeMap;

pub struct SpritesHolder {
    pub sprite_map: BTreeMap<&'static str, Handle<SpriteSheet>>,
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

pub fn initialise_graphics(world: &mut World) -> BTreeMap<&'static str, Handle<SpriteSheet>> {
    let mut spritesheet_map = BTreeMap::new();

    //ASTEROID
    spritesheet_map.insert(
        "asteroid",
        load_spritesheet(world, "texture/asteroid.png", "texture/asteroid_sheet.ron"),
    );

    //LASER
    spritesheet_map.insert(
        "laser",
        load_spritesheet(world, "texture/laser.png", "texture/laser_sheet.ron"),
    );

    //MINIGUN
    spritesheet_map.insert(
        "minigun",
        load_spritesheet(world, "texture/minigun.png", "texture/minigun_sheet.ron"),
    );

    //PLASMA
    spritesheet_map.insert(
        "plasma",
        load_spritesheet(world, "texture/plasma.png", "texture/plasma_sheet.ron"),
    );

    //PROTON
    spritesheet_map.insert(
        "proton",
        load_spritesheet(world, "texture/proton.png", "texture/proton_sheet.ron"),
    );
    //EXHAUST
    spritesheet_map.insert(
        "exhaust",
        load_spritesheet(world, "texture/exhaust.png", "texture/exhaust_sheet.ron"),
    );

    //POWER UPS
    spritesheet_map.insert(
        "powerup",
        load_spritesheet(world, "texture/powerup.png", "texture/powerup_sheet.ron"),
    );

    //EXPLOSIONS
    spritesheet_map.insert(
        "explosion_1",
        load_spritesheet(
            world,
            "texture/explosion.png",
            "texture/explosion_1_sheet.ron",
        ),
    );
    spritesheet_map.insert(
        "explosion_2",
        load_spritesheet(
            world,
            "texture/explosion.png",
            "texture/explosion_2_sheet.ron",
        ),
    );

    //PLAYERS
    spritesheet_map.insert(
        "player_1",
        load_spritesheet(world, "texture/player.png", "texture/player_1_sheet.ron"),
    );
    spritesheet_map.insert(
        "player_2",
        load_spritesheet(world, "texture/player.png", "texture/player_2_sheet.ron"),
    );

    //ENEMIES
    spritesheet_map.insert(
        "enemy_1",
        load_spritesheet(world, "texture/enemy.png", "texture/enemy_1_sheet.ron"),
    );
    spritesheet_map.insert(
        "enemy_2",
        load_spritesheet(world, "texture/enemy.png", "texture/enemy_2_sheet.ron"),
    );
    spritesheet_map.insert(
        "enemy_3",
        load_spritesheet(world, "texture/enemy.png", "texture/enemy_3_sheet.ron"),
    );
    spritesheet_map.insert(
        "enemy_4",
        load_spritesheet(world, "texture/enemy.png", "texture/enemy_4_sheet.ron"),
    );
    spritesheet_map.insert(
        "enemy_5",
        load_spritesheet(world, "texture/enemy.png", "texture/enemy_5_sheet.ron"),
    );
    spritesheet_map.insert(
        "enemy_6",
        load_spritesheet(world, "texture/enemy.png", "texture/enemy_6_sheet.ron"),
    );

    spritesheet_map
}

pub fn get_spritesheet_handle(world: &mut World, spritesheet_name: &str) -> Handle<SpriteSheet> {
    let resource = world.fetch::<SpritesHolder>();
    resource.sprite_map.get(spritesheet_name).unwrap().clone()
}
