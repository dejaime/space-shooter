use amethyst::core::{Transform, math::Vector3};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

use crate::component::Player;
use crate::component::PlayerSeat;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let (horizontal_movement, vertical_movement) = match player.seat {
                PlayerSeat::P1 => (input.axis_value("p1_horizontal").unwrap_or(0.0), input.axis_value("p1_vertical").unwrap_or(0.0)),
                PlayerSeat::P2 => (input.axis_value("p2_horizontal").unwrap_or(0.0), input.axis_value("p2_vertical").unwrap_or(0.0)),
            };
            println!("{:?} {:?}", horizontal_movement, vertical_movement);
            //TODO: Move
        }
    }
}