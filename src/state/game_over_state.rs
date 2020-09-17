use amethyst::prelude::*;

pub struct GameOverState;

impl SimpleState for GameOverState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        use crate::state::LoadingState;
        return Trans::Replace(Box::new(LoadingState {
            ..Default::default()
        }));
    }
}
