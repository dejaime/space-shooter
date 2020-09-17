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
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {    
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

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


        Trans::None
        // Trans::Push(Box::new(SpaceState {
        //     ..Default::default()
        // }))
    }
}
