use amethyst::core::{math::Vector2, timing::Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

use crate::component::Player;
use crate::component::PlayerSeat;
use crate::component::Ship;

const Y_MAX: f32 = 432.0;
const Y_MIN: f32 = -432.0;
const X_MAX: f32 = 766.0;
const X_MIN: f32 = -766.0;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, players, ships, input, time): Self::SystemData) {
        for (player, ship, transform) in (&players, &ships, &mut transforms).join() {
            let (horizontal_movement, vertical_movement) = match player.seat {
                PlayerSeat::P1 => (
                    input.axis_value("p1_horizontal").unwrap_or(0.0),
                    input.axis_value("p1_vertical").unwrap_or(0.0),
                ),
                PlayerSeat::P2 => (
                    input.axis_value("p2_horizontal").unwrap_or(0.0),
                    input.axis_value("p2_vertical").unwrap_or(0.0),
                ),
            };

            if horizontal_movement * horizontal_movement < 0.1 && vertical_movement * vertical_movement < 0.1 {
                return;
            }

            //TODO: Move
            let direction = Vector2::new(horizontal_movement, vertical_movement).normalize()
                * ship.speed
                * time.delta_seconds();

            println!("{:?} || {:?}", transform.translation(), direction);

            transform.move_right(direction.x);
            transform.move_up(direction.y);

        }
    }
}
