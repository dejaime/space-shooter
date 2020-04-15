use amethyst::prelude::*;

pub struct LoadingState;

impl SimpleState for LoadingState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}