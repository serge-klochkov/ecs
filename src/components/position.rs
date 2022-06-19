pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub velocity: Velocity,
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl PositionComponent {
    pub fn new(pos_x: f32, pos_y: f32, vel_x: f32, vel_y: f32) -> Self {
        Self {
            x: pos_x,
            y: pos_y,
            velocity: Velocity { x: vel_x, y: vel_y },
        }
    }

    pub fn apply_velocity(&mut self) {
        self.x += self.velocity.x;
        self.y += self.velocity.y;
        // println!("{}, {}", self.x, self.y)
    }
}
