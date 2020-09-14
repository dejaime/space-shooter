use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, Write},
};

//#[derive(SystemDesc)]
pub struct GameOverSystem;

use crate::component::Player;
use crate::component::PlayerSeat;
use crate::component::Health;


pub struct DeadPlayers {
    pub p1_dead: bool,
    pub p2_dead: bool,
}

impl Default for DeadPlayers {
    fn default() -> Self {
        DeadPlayers {
            p1_dead: false,
            p2_dead: false
        }
    }
}

impl<'s> System<'s> for GameOverSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Health>,
        Write<'s, DeadPlayers>,
    );


    fn run(
        &mut self,
        (players, healths, mut dead_players_resource): Self::SystemData,
    ) {
        for (player, health) in (&players, &healths).join() {
            if health.health <= 0.0 {
                if player.seat == PlayerSeat::P1 {
                    dead_players_resource.p1_dead = true;
                } else {
                    dead_players_resource.p2_dead = true;
                }
            } else {
                if player.seat == PlayerSeat::P1 {
                    dead_players_resource.p1_dead = false;
                } else {
                    dead_players_resource.p2_dead = false;
                }
            }
        }        
    }
}
