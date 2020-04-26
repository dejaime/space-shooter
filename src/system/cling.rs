use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, ReadStorage, WriteStorage},
};

use crate::component::clingy_component::ClingTo;

pub struct ClingSystem;

impl<'a> System<'a> for ClingSystem {
    type SystemData = (
        ReadStorage<'a, ClingTo>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (clingies, transforms): Self::SystemData) {
        for (clingy, transform) in (&clingies, &transforms).join() {
            /*  pub target: Entity,
                pub offset: Vector3<f32>,
                pub max_distance: f32,
                pub speed: f32,
                pub perfect_follow: bool,*/
            
            if clingy.perfect_follow {
                //MOVE INSTANTLY
                continue;
            } else {
                //APPLY SPEED
            }
        }
    }
}
