pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Self { x, y, vx, vy }
    }

    pub fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}
