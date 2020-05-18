use amethyst::{
    core::Transform,
    ecs::{System, ReadStorage, WriteStorage},
};

use crate::component::clingy_component::ClingTo;

pub struct ClingSystem;

impl<'a> System<'a> for ClingSystem {
    type SystemData = (
        ReadStorage<'a, ClingTo>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (_clingies, _transforms): Self::SystemData) {
        //for (_clingy, _transform) in (&clingies, &transforms).join() {
            /*  pub target: Entity,
                pub offset: Vector3<f32>,
                pub max_distance: f32,
                pub speed: f32,
                pub perfect_follow: bool,*/

                //APPLY SPEED
                //continue;
        //}
    }
}
