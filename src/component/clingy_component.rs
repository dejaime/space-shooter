#[derive(Component)]
pub struct ClingTo {
    target: Entity,
    max_distance: f32,
    speed: f32,
}
