use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::component::{
    enemy_component::Enemy,
    player_component::{Player, PlayerSeat},
};

pub struct ClingSystem;

impl<'s> System<'s> for ClingSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Health>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
    );

    fn run(&mut self, (entities, mut healths, players, enemies): Self::SystemData) {
        for (health, players) in (&healths, &players).join() {
            continue;
        }

        for (health, enemies) in (&healths, &enemies).join() {
            continue;
        }
    }
}
