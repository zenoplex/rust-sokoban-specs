use crate::components::*;
use specs::{Builder, Entity, World, WorldExt};

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_static(String::from("/images/wall.png")))
        .with(Wall {})
        .with(Immovable {})
        .build()
}

pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable::new_static(String::from("/images/floor.png")))
        .build()
}

pub fn create_box(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            format!("/images/box_{}_1.png", color),
            format!("/images/box_{}_2.png", color),
        ]))
        .with(Box { color })
        .with(Movable {})
        .build()
}

pub fn create_box_spot(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_static(format!(
            "/images/box_spot_{}.png",
            color
        )))
        .with(BoxSpot { color })
        .build()
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            String::from("/images/player_1.png"),
            String::from("/images/player_2.png"),
            String::from("/images/player_3.png"),
        ]))
        .with(Player {})
        .with(Movable {})
        .build()
}
