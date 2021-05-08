use crate::{
    components::{Box, BoxSpot, Position},
    resources::{Gameplay, GameplayState},
};
use specs::{Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, boxes, box_spots, positions) = data;

        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect();

        for (box_spot, position) in (&box_spots, &positions).join() {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if the_box.color == box_spot.color {
                    // We don't set GameplayState::Won because there could be multiple boxes
                    // We could keep track of box count / map
                } else {
                    return;
                }
            } else {
                gameplay.state = GameplayState::Playing;
                return;
            }
        }

        gameplay.state = GameplayState::Won;
    }
}
