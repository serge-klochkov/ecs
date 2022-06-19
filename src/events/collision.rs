pub struct CollisionEvent {
    pub x_offset: f32,
    pub y_offset: f32,
    pub entity_id: usize,
    pub other_entity_id: Option<usize>,
}
