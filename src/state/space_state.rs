use amethyst::{core::timing::Time, prelude::*};

use crate::entity::player_ship::spawn_player_ship;

#[derive(Default)]
pub struct SpaceState {
    pub player_one_lives: u8,
    pub player_two_lives: u8,

    pub player_one_score: u64,
    pub player_two_score: u64,

    pub player_one_distance_travelled: u32,
    pub player_two_distance_travelled: u32,

    pub player_one_projectiles_fired: u64,
    pub player_two_projectiles_fired: u64,

    pub player_one_damage_dealt: u64,
    pub player_two_damage_dealt: u64,

    pub player_one_kills: u32,
    pub player_two_kills: u32,

    pub open_menu_timer: Option<f32>,
}

impl SimpleState for SpaceState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.player_one_lives = 3;
        self.player_two_lives = 3;

        self.player_one_score = 0;
        self.player_two_score = 0;

        self.player_one_distance_travelled = 0;
        self.player_two_distance_travelled = 0;

        self.player_one_projectiles_fired = 0;
        self.player_two_projectiles_fired = 0;

        self.player_one_damage_dealt = 0;
        self.player_two_damage_dealt = 0;

        self.player_one_kills = 0;
        self.player_two_kills = 0;

        self.open_menu_timer.replace(1.0);


        spawn_player_ship(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.open_menu_timer.take() {
            timer -= data.world.fetch::<Time>().delta_seconds();
            if timer <= 0.0 {
                // Do it
            } else {
                // Timer is not expired yet, putting it back
                self.open_menu_timer.replace(timer);
            }
        }
        Trans::None
    }
}
