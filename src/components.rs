use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;

/// Velocity -> Position
/// then render: Sprite

pub struct VelocityComponent {
    pub x: f32,
    pub y: f32,
}

impl VelocityComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }
}

impl VelocityComponent {}

pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

impl PositionComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    pub fn update(&mut self, velocity: &VelocityComponent) {
        self.x += velocity.x;
        self.y += velocity.y;
    }
}

pub struct SpriteComponent {
    pub texture_id: usize,
    pub sprite: Rect,
}

impl SpriteComponent {
    pub fn new(texture_id: usize, sprite: Rect) -> Self {
        Self { texture_id, sprite }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        position: &PositionComponent,
        textures: &HashMap<usize, Texture>,
    ) {
        let texture = textures
            .get(&self.texture_id)
            .expect("Failed to get texture");

        let screen_position = position.point();
        let screen_rect =
            Rect::from_center(screen_position, self.sprite.width(), self.sprite.height());
        canvas
            .copy(texture, self.sprite, screen_rect)
            .expect("Failed to copy texture")
    }
}
