use amethyst::ecs::prelude::{Component, VecStorage};

#[derive(Debug, Copy, Clone)]
pub enum PlayerSeat {
    NonPlayer,
    P1,
    P2,
}

#[derive(Debug)]
pub struct Player {
    pub seat: PlayerSeat,
    pub hit_invincibility_timer: f32,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

impl Default for Player {
    fn default() -> Self {
        Player {
            seat: PlayerSeat::NonPlayer,
            hit_invincibility_timer: 0.0,
        }
    }
}
