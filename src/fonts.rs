use amethyst::{
    assets::{Loader},
    ui::{FontHandle, TtfFormat},
    prelude::*,
};

pub struct FontHolder {
    pub font_handle: FontHandle
}

pub fn initialise_fonts(world: &mut World) -> FontHandle {
    world.read_resource::<Loader>().load(
        "font/PixelOperator.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    )
}

pub fn get_font(world: &mut World) -> FontHandle {
    let font = world.fetch::<FontHolder>();
    font.font_handle.clone()
}
