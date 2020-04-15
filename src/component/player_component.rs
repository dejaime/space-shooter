use amethyst::ecs::prelude::{Component, VecStorage};

pub struct PlayerSeat {
    pub P1,
    pub P2
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub seat: PlayerSeat;
}
