use amethyst::{
    core::{
        math::{Vector2, Vector3},
        timing::Time,
        Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct PlayerMovementSystem;

use crate::component::Player;
use crate::component::PlayerSeat;
use crate::component::Ship;

const Y_MAX: f32 = 400.0;
const Y_MIN: f32 = -400.0;
const X_MAX: f32 = 800.0;
const X_MIN: f32 = -800.0;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut sprite_renders, players, ships, input, time): Self::SystemData,
    ) {
        for (player, ship, sprite_render, transform) in
            (&players, &ships, &mut sprite_renders, &mut transforms).join()
        {
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

            if horizontal_movement * horizontal_movement < 0.1
                && vertical_movement * vertical_movement < 0.1
            {
                sprite_render.sprite_number = 0;
                continue;
            }

            let direction = Vector2::new(horizontal_movement, vertical_movement).normalize();
            //This entire if block serves to decide which sprite to render (flat, turning, strong turning)
            if direction.x * direction.x > 0.3 {
                if direction.x * direction.x > 0.7 {
                    sprite_render.sprite_number = 2;
                } else {
                    sprite_render.sprite_number = 1;
                }

                if direction.x > 0.0 {
                    transform.set_scale(Vector3::new(-1.0, 1.0, 1.0));
                } else {
                    transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
                }
            } else {
                sprite_render.sprite_number = 0;
            }

            let direction = direction * ship.speed * time.delta_seconds();

            transform.set_translation_x(
                (transform.translation().x + direction.x)
                    .min(X_MAX)
                    .max(X_MIN),
            );
            transform.set_translation_y(
                (transform.translation().y + direction.y)
                    .min(Y_MAX)
                    .max(Y_MIN),
            );
        }
    }
}
