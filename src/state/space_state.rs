use amethyst::prelude::*;

pub struct SpaceState;

impl SimpleState for SpaceState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}