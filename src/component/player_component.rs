use amethyst::ecs::prelude::{Component, VecStorage};


pub enum PlayerSeat {
    P1,
    P2
}

pub struct Player {
    pub seat: PlayerSeat,
    pub hit_invincibility_timer: f32,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
