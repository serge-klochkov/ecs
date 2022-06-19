pub struct SpriteComponent {
    pub texture_id: usize,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl SpriteComponent {
    pub fn new(texture_id: usize, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            texture_id,
            x,
            y,
            w,
            h,
        }
    }
}
