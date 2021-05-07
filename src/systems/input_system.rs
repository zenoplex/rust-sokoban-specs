use crate::components::*;
use crate::constants::*;
use crate::resources::InputQueue;
use ggez::event;
use specs::world::Index;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                // Get all positions
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect();

                let (start, end, is_x) = match key {
                    event::KeyCode::Up => (position.y, 0, false),
                    event::KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    event::KeyCode::Left => (position.x, 0, true),
                    event::KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                // Creating range to limit search range
                let range: Vec<u8> = if start < end {
                    (start..=end).collect()
                } else {
                    (end..=start).rev().collect()
                };

                println!("range {:?}", &range);

                // iterate possible affecting entities in player movement direction
                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    println!("pos to search: {:?}", &pos);

                    match mov.get(&pos) {
                        // id.clone() maybe better?
                        Some(id) => to_move.push((key, *id)),
                        None => match immov.get(&pos) {
                            Some(_id) => to_move.clear(),
                            // if there are no related immovable exit loop
                            None => break,
                        },
                    }
                }
            }
        }

        if !to_move.is_empty() {
            println!("to_move: {:?}", to_move);
        }

        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
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
