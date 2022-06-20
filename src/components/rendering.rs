use sdl2::pixels::Color;

pub enum RenderingComponent {
    Sprite(Sprite),
    Rectangle(Rectangle),
}

pub struct Sprite {
    pub texture_id: usize,
    pub w: u32,
    pub h: u32,
}

pub struct Rectangle {
    pub w: u32,
    pub h: u32,
    pub color: Color,
    pub border: Option<Border>,
}
pub struct Border {
    pub border_size: u32,
    pub border_color: Color,
}

impl RenderingComponent {
    pub fn new_sprite(texture_id: usize, w: u32, h: u32) -> Self {
        RenderingComponent::Sprite(Sprite { texture_id, w, h })
    }
    pub fn new_rectangle(w: u32, h: u32, color: Color, border: Option<Border>) -> Self {
        RenderingComponent::Rectangle(Rectangle {
            w,
            h,
            color,
            border,
        })
    }
}
