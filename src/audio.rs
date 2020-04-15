use amethyst::{
    assets::{Loader, AssetStorage},
    audio::{output::Output, Source, OggFormat, SourceHandle, AudioSink},
    ecs::{World, WorldExt},
};

use std::{iter::Cycle, vec::IntoIter};

// const BOUNCE_SOUND: &str = "audio/bounce.ogg";
// const SCORE_SOUND: &str = "audio/score.ogg";

const MUSIC_FILES: &[&str] = &[
    "audio/GalacticTemple.ogg",
    "audio/ObservingTheStar.ogg",
    "audio/bleeding_out2.ogg",
    "audio/through space.ogg",
];


pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}



fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialise_music(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let music = MUSIC_TRACKS
            .iter()
            .map(|music_file| load_audio_track(&loader, &world, music_file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        music
    };
    world.insert(music);
}
