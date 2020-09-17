use amethyst::{
    ui::{UiTransform, UiText, LineMode, Anchor, Interactable},
    core::timing::Time, 
    prelude::*,
};

use crate::entity::{
    prop::{spawn_prop, prop_warm_up}
};


use rand::prelude::*;

use crate::state::PropCounter;

use crate::fonts::get_font;

pub struct MenuState {
    pub spawn_prop_timer: Option<f32>,
    pub rng: ThreadRng,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {    
            spawn_prop_timer: Some(1.0),
            rng: thread_rng(),
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        self.rng = thread_rng();

        prop_warm_up(data.world, &mut self.rng);

        data.world.insert(PropCounter {
            ..Default::default()
        });

        let one_player_button_uitransform = UiTransform::new(
            String::from("1p_btn"),
            Anchor::Middle,
            Anchor::Middle,
            0f32, 48f32, 0f32, 100f32, 30f32,
        );

        let two_player_button_uitransform = UiTransform::new(
            String::from("2p_btn"),
            Anchor::Middle,
            Anchor::Middle,
            0f32, 0f32, 0f32, 100f32, 30f32,
        );

        let one_uitext = UiText::new(
            get_font(data.world),
            String::from("1 Player"),
            [1.0, 1.0, 1.0, 0.5],
            25f32,
            LineMode::Single,
            Anchor::Middle,
        );

        let two_uitext = UiText::new(
            get_font(data.world),
            String::from("2 Players"),
            [1.0, 1.0, 1.0, 0.5],
            25f32,
            LineMode::Single,
            Anchor::Middle,
        );

        let _ = data.world.create_entity()
            .with(one_player_button_uitransform)
            .with(one_uitext)
            .with(Interactable)
            .build();
        
        let _ = data.world.create_entity()
            .with(two_player_button_uitransform)
            .with(two_uitext)
            .with(Interactable)
            .build();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        let delta_time = data.world.fetch::<Time>().delta_seconds();

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

        Trans::None
        // Trans::Push(Box::new(SpaceState {
        //     ..Default::default()
        // }))
    }
}
