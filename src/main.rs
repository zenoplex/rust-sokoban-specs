use ggez::{conf, event, graphics, input, nalgebra, Context, ContextBuilder, GameResult};
use specs::{
    Builder, Component, Entity, Join, ReadStorage, RunNow, System, VecStorage, World, WorldExt,
    Write, WriteStorage,
};
use std::path;

const TILE_WIDTH: f32 = 32.0;

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct Renderable {
    path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
struct BoxSpot {}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
}

fn create_wall(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/images/wall.png"),
        })
        .with(Wall {})
        .build()
}

fn create_floor(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: String::from("/images/floor.png"),
        })
        .build()
}

fn create_box(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/images/box.png"),
        })
        .with(Box {})
        .build()
}

fn create_box_spot(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/images/box_spot.png"),
        })
        .with(BoxSpot {})
        .build()
}

fn create_player(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/images/player.png"),
        })
        .with(Player {})
        .build()
}

struct RenderingSystem<'a> {
    context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data: Vec<_> = (&positions, &renderables).join().collect();
        // sort by z-index
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image =
                graphics::Image::new(self.context, renderable.path.clone()).expect("Image error");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = graphics::DrawParam::new().dest(nalgebra::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("Draw error");
        }

        graphics::present(self.context).expect("Render error");
    }
}

struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, players) = data;

        for (position, _player) in (&mut positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                match key {
                    event::KeyCode::Up => position.y -= 1,
                    event::KeyCode::Down => position.y += 1,
                    event::KeyCode::Left => position.x -= 1,
                    event::KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}

#[derive(Default)]
struct InputQueue {
    keys_pressed: Vec<event::KeyCode>,
}

fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

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

    fn update(&mut self, _context: &mut Context) -> GameResult {
        let mut input_system = InputSystem {};
        input_system.run_now(&self.world);
        Ok(())
        // todo!()
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rendering_system = RenderingSystem { context };
        rendering_system.run_now(&self.world);
        Ok(())
    }
}

fn load_map(world: &mut World, map_string: &str) {
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };

            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position);
                }
                "N" => (),
                char => panic!("Unrecognized map item {}", char),
            }
        }
    }

    println!("{:?}", rows);
}

fn initialize_level(world: &mut World) {
    let map = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
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
