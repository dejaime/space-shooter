use amethyst::prelude::*;

pub struct GameOverState;

impl SimpleState for GameOverState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}