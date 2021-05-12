pub type EntityId = u32;

#[derive(Debug)]
pub struct EntityMoved {
    pub id: EntityId,
}

#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool,
}

#[derive(Debug)]
pub enum Event {
    PlayerHitObstacle,
    EntityMoved(EntityMoved),
    BoxPlacedOnSpot(BoxPlacedOnSpot),
}
