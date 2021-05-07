use ggez::event;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<event::KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}
