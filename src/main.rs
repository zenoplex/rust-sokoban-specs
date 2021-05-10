use ggez::{conf, event, timer, Context, ContextBuilder, GameResult};
use specs::{RunNow, World, WorldExt};

mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::components::register_components;
use crate::map::load_map;
use crate::resources::{register_resources, InputQueue, Time};
use crate::systems::{GameplayStateSystem, InputSystem, RenderingSystem};
use std::path;

struct Game {
    world: World,
}

impl event::EventHandler for Game {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }

    fn update(&mut self, context: &mut Context) -> GameResult {
        let mut input_system = InputSystem {};
        input_system.run_now(&self.world);

        let mut gameplay_state_system = GameplayStateSystem {};
        gameplay_state_system.run_now(&self.world);

        let mut time = self.world.write_resource::<Time>();
        time.delta += timer::delta(context);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rendering_system = RenderingSystem { context };
        rendering_system.run_now(&self.world);
        Ok(())
    }
}

fn initialize_level(world: &mut World) {
    let map = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, map);
}

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    let context_builder = ContextBuilder::new("rust_sokoban", "zenoplex")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, mut event_loop) = context_builder.build()?;
    let game = &mut Game { world };
    event::run(&mut context, &mut event_loop, game)
}
