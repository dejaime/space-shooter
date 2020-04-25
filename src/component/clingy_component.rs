use amethyst::core::math::Vector3;

#[derive(Component)]
pub struct ClingTo {
    target: Entity,
    offset: Vector3,
    max_distance: f32,
    speed: f32,
    perfect_follow: bool,
}
