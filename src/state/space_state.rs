use amethyst::{core::timing::Time, prelude::*};

pub struct SpaceState {
    score: u64,
    distance_travelled: u64,
    projectiles_fired: u64,
    damage_dealt: u64,
    kills: u64,

}

impl SimpleState for SpaceState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}