use amethyst::{
    core::timing::Time,
    ecs::world::*,
    prelude::*,
    ui::{Anchor, Interactable, LineMode, UiText, UiTransform, UiEventType},
};

use crate::entity::prop::{prop_warm_up, spawn_prop};
use crate::state::SpaceState;

use rand::prelude::*;

use crate::state::PropCounter;

use crate::fonts::get_font;

pub struct MenuState {
    pub spawn_prop_timer: Option<f32>,
    pub rng: ThreadRng,
    pub p1_button: Option<Entity>,
    pub p2_button: Option<Entity>,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {
            spawn_prop_timer: Some(1.0),
            rng: thread_rng(),
            p1_button: None,
            p2_button: None,
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Menu on_start");
        self.rng = thread_rng();

        prop_warm_up(data.world, &mut self.rng);

        data.world.insert(PropCounter {
            ..Default::default()
        });

        let one_player_button_uitransform = UiTransform::new(
            String::from("1p_btn"),
            Anchor::Middle,
            Anchor::Middle,
            0f32,
            48f32,
            0f32,
            100f32,
            30f32,
        );

        let two_player_button_uitransform = UiTransform::new(
            String::from("2p_btn"),
            Anchor::Middle,
            Anchor::Middle,
            0f32,
            0f32,
            0f32,
            100f32,
            30f32,
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

        let p1_button = data
            .world
            .create_entity()
            .with(one_player_button_uitransform)
            .with(one_uitext)
            .with(Interactable)
            .build();

        self.p1_button = Some(p1_button);

        let p2_button = data
            .world
            .create_entity()
            .with(two_player_button_uitransform)
            .with(two_uitext)
            .with(Interactable)
            .build();
        self.p2_button = Some(p2_button);

        data.world.maintain();
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

        
        data.world.maintain();
        Trans::None
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let mut next_state = SimpleTrans::None;

    	if let StateEvent::Ui(ui_event) = event {
            let p1_is_target = ui_event.target == self.p1_button.unwrap();
            let p2_is_target = ui_event.target == self.p2_button.unwrap();

    		match ui_event.event_type {
    			UiEventType::Click if p1_is_target || p2_is_target => {

                    let p1_button = self
                        .p1_button
                        .expect("Failed deleting main menu button... somehow");
                    let p2_button = self
                        .p2_button
                        .expect("Failed deleting main menu button... somehow");

                    data.world
                        .delete_entity(p1_button)
                        .expect("Failed to delete entity. Was it already removed?");
                    data.world
                        .delete_entity(p2_button)
                        .expect("Failed to delete entity. Was it already removed?");
                    next_state = Trans::Replace(Box::new(SpaceState {
                            ..Default::default()
                        }))
                },
                
    			_ => {
    				next_state = SimpleTrans::None
    			},  
            };
        }
        
        next_state
    }
}
