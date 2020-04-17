use amethyst::core::{Transform, timing::Time, math::Vector2};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

use crate::component::Ship;
use crate::component::Player;
use crate::component::PlayerSeat;

const Y_MAX:f32 = 1080.0;
const Y_MIN:f32 = 0.0;
const X_MAX:f32 = 1920.0;
const X_MIN:f32 = 0.0;

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
                PlayerSeat::P1 => (input.axis_value("p1_horizontal").unwrap_or(0.0), input.axis_value("p1_vertical").unwrap_or(0.0)),
                PlayerSeat::P2 => (input.axis_value("p2_horizontal").unwrap_or(0.0), input.axis_value("p2_vertical").unwrap_or(0.0)),
            };
            println!("{:?} {:?}", horizontal_movement, vertical_movement);
            //TODO: Move
            let direction = Vector2::new(horizontal_movement, vertical_movement).normalize() * ship.speed * time.delta_seconds();
            let current_position = (transform.translation().x, transform.translation().y);

            transform.set_translation_x(
                (current_position.0 + direction.x)
                    .min(X_MIN)
                    .max(X_MAX)
            );

            transform.set_translation_y(
                (current_position.0 + direction.y)
                    .min(Y_MIN)
                    .max(Y_MAX)
            );
        }
    }
}
