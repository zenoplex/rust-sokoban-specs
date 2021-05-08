use crate::components::*;
use specs::{Builder, Entity, World, WorldExt};

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/images/wall.png"),
        })
        .with(Wall {})
        .with(Immovable {})
        .build()
}

pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: String::from("/images/floor.png"),
        })
        .build()
}

pub fn create_box(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: format!("/images/box_{}.png", color),
        })
        .with(Box { color })
        .with(Movable {})
        .build()
}

pub fn create_box_spot(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: format!("/images/box_spot_{}.png", color),
        })
        .with(BoxSpot { color })
        .build()
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: String::from("/images/player.png"),
        })
        .with(Player {})
        .with(Movable {})
        .build()
}
