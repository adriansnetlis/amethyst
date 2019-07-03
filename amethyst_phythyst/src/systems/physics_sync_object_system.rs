use crate::{objects::*, servers::RBodyPhysicsServer};
use amethyst_core::{
    ecs::{Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage, Resources, storage::ComponentEvent, ReaderId},
    transform::components::Transform,
};

pub struct PhysicsSyncObjectSystem{
    event_reader_id: Option<ReaderId<ComponentEvent>>,
}

impl PhysicsSyncObjectSystem{
    pub fn new() -> Self {
        PhysicsSyncObjectSystem{
            event_reader_id: None,
        }
    }

    fn setup_step_2(&mut self, res: &mut Resources){

        // Register the event reader of the PhysicsBodyTag storage
        self.event_reader_id = Some(WriteStorage::<PhysicsBodyTag>::fetch(res).register_reader());
    }
}

impl<'s> System<'s> for PhysicsSyncObjectSystem {
    type SystemData = (
        ReadStorage<'s, PhysicsBodyTag>,
    );

    fn run(&mut self, (bodies): Self::SystemData) {
        let events = bodies.channel().read(self.event_reader_id.as_mut().unwrap());

        for event in events {
            match event {
                ComponentEvent::Removed(index) => {

                }
                _ => {}
            }
        }
    }

    define_setup_with_physics_assertion!(setup_step_2);

}