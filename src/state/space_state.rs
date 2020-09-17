use amethyst::{core::timing::Time, prelude::*};

use crate::entity::{
    enemy_ship::spawn_simple_enemy, player_ship::spawn_player_ship, prop::spawn_prop,
};

use crate::state::GameOverState;
use crate::system::DeadPlayers;

use rand::prelude::*;

pub struct SpaceState {
    pub player_one_lives: u8,
    pub player_two_lives: u8,

    pub player_one_score: u64,
    pub player_two_score: u64,

    pub player_one_projectiles_fired: u64,
    pub player_two_projectiles_fired: u64,

    pub player_one_damage_dealt: u64,
    pub player_two_damage_dealt: u64,

    pub player_one_kills: u32,
    pub player_two_kills: u32,

    pub spawn_prop_timer: Option<f32>,
    pub running_intro: bool,
    pub game_over: bool,

    pub rng: ThreadRng,
}

impl Default for SpaceState {
    fn default() -> Self {
        SpaceState {
            player_one_lives: 3,
            player_two_lives: 3,

            player_one_score: 0,
            player_two_score: 0,

            player_one_projectiles_fired: 0,
            player_two_projectiles_fired: 0,

            player_one_damage_dealt: 0,
            player_two_damage_dealt: 0,

            player_one_kills: 0,
            player_two_kills: 0,

            spawn_prop_timer: Some(1.0),
            running_intro: true,
            game_over: false,

            rng: thread_rng(),
        }
    }
}

pub struct PropCounter {
    pub total_spawned_props: u64,
    pub active_props_count: u64,
}

impl Default for PropCounter {
    fn default() -> Self {
        PropCounter {
            total_spawned_props: 0,
            active_props_count: 0,
        }
    }
}

impl SimpleState for SpaceState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        spawn_player_ship(data.world, false);
        spawn_player_ship(data.world, true);

        spawn_simple_enemy(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let delta_time = data.world.fetch::<Time>().delta_seconds();

        if self.game_over {
            data.world.delete_all();
            return Trans::Replace(Box::new(GameOverState {}));
        }
        let (p1_dead, p2_dead) = {
            let dead_players = data.world.fetch::<DeadPlayers>();
            (dead_players.p1_dead, dead_players.p2_dead)
        };

        if self.running_intro {
            //TODO Run Intro (Push intro state, to be popped later)
            self.running_intro = false;
        } else {
            if p1_dead && self.player_one_lives > 0 {
                self.player_one_lives -= 1;
                spawn_player_ship(data.world, false);
            }

            if p2_dead && self.player_two_lives > 0 {
                self.player_two_lives -= 1;
                spawn_player_ship(data.world, true);
            } else if p1_dead && p2_dead && self.player_one_lives <= 0 {
                self.game_over = true;
            }

            if let Some(mut timer) = self.spawn_prop_timer.take() {
                timer -= delta_time;
                if timer <= 0.0 {
                    {
                        let mut prop_counter = data.world.fetch_mut::<PropCounter>();
                        prop_counter.active_props_count += 1;
                        prop_counter.total_spawned_props += 1;
                    }
                    spawn_prop(data.world, &mut self.rng);
                    self.spawn_prop_timer.replace(self.rng.gen_range(0.01, 0.3));
                } else {
                    self.spawn_prop_timer.replace(timer);
                }
            }
        }

        data.world.maintain();

        Trans::None
    }
}
