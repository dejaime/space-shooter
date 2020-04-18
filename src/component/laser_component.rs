use amethyst::{
    ecs::prelude::{Component, VecStorage},
    core::math::Vector3,
};

#[derive(Debug)]
pub struct Laser {
    pub directional_speed: Vector3<f32>,
    pub directional_acceleration: Vector3<f32>,
    pub directional_torque: Vector3<f32>,
    pub rotational_radian_speed: f32,
    pub radius: f32,
    pub destroy_on_hit: bool,
}

impl Component for Laser {
    type Storage = VecStorage<Self>;
}
