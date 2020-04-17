use amethyst::{
    core::timing::Time,
    prelude::*,
};

use std::collections::VecDeque;

use crate::entity::{player_ship::spawn_player_ship, prop::spawn_prop};

use rand::prelude::*;

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

    pub spawn_prop_timer: Option<f32>,
    pub running_intro: bool,

    pub last_deltas: VecDeque<f32>,
    pub rng: ThreadRng,
}

impl Default for SpaceState {
    fn default() -> Self {
        SpaceState {
            player_one_lives: 3,
            player_two_lives: 3,

            player_one_score: 0,
            player_two_score: 0,

            player_one_distance_travelled: 0,
            player_two_distance_travelled: 0,

            player_one_projectiles_fired: 0,
            player_two_projectiles_fired: 0,

            player_one_damage_dealt: 0,
            player_two_damage_dealt: 0,

            player_one_kills: 0,
            player_two_kills: 0,

            spawn_prop_timer: Some(1.0),
            running_intro: true,

            last_deltas: VecDeque::new(),

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
        for _ in 0..120 {
            self.last_deltas.push_front(0.015);
        }
        spawn_player_ship(data.world, false);
        spawn_player_ship(data.world, true);

        data.world.insert(PropCounter {
            ..Default::default()
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let delta_time = data.world.fetch::<Time>().delta_seconds();
        self.last_deltas.pop_back();
        self.last_deltas.push_front(delta_time);

        let mut frame_times = 0.0;
        for i in 0..120 {
            frame_times += self.last_deltas[i];
        }

        if self.running_intro {
            self.running_intro = false;
        } else {
            if let Some(mut timer) = self.spawn_prop_timer.take() {
                timer -= delta_time;
                if timer <= 0.0 {
                    {
                        let mut prop_counter = data.world.fetch_mut::<PropCounter>();
                        prop_counter.active_props_count += 1;
                        prop_counter.total_spawned_props += 1;
                    }
                    spawn_prop(data.world, &mut self.rng);
                    self.spawn_prop_timer
                        .replace(self.rng.gen_range(0.01, 0.3));
                } else {
                    self.spawn_prop_timer.replace(timer);
                }
            }
        }

        data.world.maintain();

        Trans::None
    }
}
