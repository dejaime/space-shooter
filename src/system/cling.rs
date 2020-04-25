use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, ReadStorage, WriteStorage},
};

pub struct ClingSystem;

impl<'a> System<'a> for ClingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, ClingTo>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (entity, target, transform): Self::SystemData) {
        for (entity, t) in (&*entity, &target).join() {
            *transform.get_mut(entity).unwrap() = transform.get(t.target).cloned().unwrap() + t.offset;
            //TODO: Apply speed when perfect_follor is false instead of instant teleport
        }
    }
}
