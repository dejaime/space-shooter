use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod audio;
mod component;
mod entity;
mod graphics;
mod state;
mod system;

use state::LoadingState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        //Window definitions
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        //Entity Transform components and systems
        .with_bundle(TransformBundle::new())?
        //Input
        .with_bundle(input_bundle)?
        //Audio
        .with_bundle(AudioBundle::default())?
        //Background music play and selection
        .with_system_desc(
            DjSystemDesc::new(|music: &mut audio::Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(
            system::PlayerMovementSystem,
            "player_movement_system",
            &["input_system"],
        )
        .with(system::BackgroundPropSystem, "background_prop_system", &[])
        .with(system::LaserFiringSystem, "laser_firing_system", &[])
        .with(system::LaserMovementSystem, "laser_movement_system", &[])
        .with(system::LaserCollisionSystem, "laser_collision_system", &[])
        .with(system::HealthSystem, "health_system", &[])
        .with(system::GameOverSystem, "game_over_system", &[])
        .with(system::EnergyRecoverySystem, "energy_recovery_system", &[]);
    let mut game = Application::new(assets_dir, LoadingState {}, game_data)?;
    game.run();

    Ok(())
}
