use amethyst::ecs::prelude::{Component, VecStorage};

use crate::component::player_component::PlayerSeat;

#[derive(Debug)]
pub struct PlayerLaser {
    pub seat: PlayerSeat,
}

impl Component for PlayerLaser {
    type Storage = VecStorage<Self>;
}
