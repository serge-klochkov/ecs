pub struct CollisionEvent {
    pub new_x: f32,
    pub new_y: f32,
    pub new_vx: f32,
    pub new_vy: f32,
    pub entity_id: usize,
    pub other_entity_id: Option<usize>,
}
