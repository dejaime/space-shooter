use amethyst::{core::timing::Time, prelude::*};

pub struct SpaceState {
    score: u64,
    distance_travelled: u64,
    projectiles_fired: u64,
    damage_dealt: u64,
    kills: u64,

    open_menu_timer: Option<f32>,
}


impl SimpleState for SpaceState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {

        self.score = 0;
        self.distance_travelled = 0;
        self.projectiles_fired = 0;
        self.damage_dealt = 0;
        self.kills = 0;

        self.open_menu_timer.replace(1.0);
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
