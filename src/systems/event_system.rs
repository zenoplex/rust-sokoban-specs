use crate::{
    components::*,
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    resources::EventQueue,
};
use specs::{Entities, Join, ReadStorage, System, Write};
use std::collections::HashMap;

pub struct EventSystem {}

impl<'a> System<'a> for EventSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut new_events = Vec::new();

        let (mut event_queue, entities, boxes, box_spots, positions) = data;

        for event in event_queue.events.drain(..) {
            println!("New Event: {:?}", event);

            match event {
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };
                }

                Event::EntityMoved(EntityMoved { id }) => {
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_with_positions: HashMap<(u8, u8), &BoxSpot> =
                            (&positions, &box_spots)
                                .join()
                                .map(|t| ((t.0.x, t.0.y), t.1))
                                .collect();

                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            if let Some(box_spot) =
                                box_spots_with_positions.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: (box_spot.color == the_box.color),
                                }));
                            }
                        }
                    }
                }

                Event::PlayerHitObstacle => {
                    // play "wall" sound
                }
            }
        }

        event_queue.events.append(&mut new_events);
    }
}
