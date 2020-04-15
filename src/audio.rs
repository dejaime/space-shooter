use amethyst::{
    assets::{Loader, /*AssetStorage*/},
    audio::{/*output::Output, Source,*/ OggFormat, SourceHandle, /*AudioSink*/},
    ecs::{World, WorldExt},
};

use std::{fs, iter::Cycle, vec::IntoIter};
use rand::seq::SliceRandom;

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}



fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialise_music(world: &mut World) {
    let music = {
        let loader = world.read_resource::<Loader>();

        let mut tracks = Vec::new();

        let audio_files = fs::read_dir("./assets/audio/music").unwrap();
        
        for file in audio_files {
            let track_file_path = format!("{}", file.unwrap().path().display());
            let split_vec: Vec<&str> = track_file_path.split("./assets/").collect();
            let fixed_name = split_vec.last().unwrap().clone();
            if fixed_name.ends_with(".ogg") {
                tracks.push(format!("{}", fixed_name));
            } else {
                println! ("[ERROR] Invalid audio extension on file {}, only OGG is supported. Ignoring file.", fixed_name);
            }
        }

        let mut random = rand::thread_rng();
        tracks.shuffle(&mut random);

        let music = tracks
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
