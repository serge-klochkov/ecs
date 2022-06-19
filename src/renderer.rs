use std::collections::HashMap;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::Sdl;

use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::controls::Controls;

pub struct Renderer {
    context: Sdl,
    window_canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<usize, Texture>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        let context = sdl2::init().expect("Failed to init SDL context");
        let video_subsystem = context
            .video()
            .expect("Failed to initialize the video subsystem");
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
            .expect("Failed to initialize SDL2 image subsystem");
        let window = video_subsystem
            .window("Entity Component", width, height)
            .position_centered()
            .build()
            .expect("Failed to create a window");
        let window_canvas = window
            .into_canvas()
            .build()
            .expect("Failed to create a canvas");
        let texture_creator = window_canvas.texture_creator();
        Self {
            textures: HashMap::new(),
            texture_creator,
            window_canvas,
            context,
        }
    }

    pub fn init_controls(&self) -> Controls {
        let event_pump = self.context.event_pump().expect("Failed to get event pump");
        Controls::new(event_pump)
    }

    pub fn add_texture(&mut self, texture_id: usize, path: &str) {
        let texture: Texture = self
            .texture_creator
            .load_texture(path)
            .expect(&format!("Failed to load a texture {}", path));
        self.textures.insert(texture_id, texture);
    }

    pub fn clear_canvas(&mut self) {
        let color = Color::RGB(100, 64, 255);
        self.window_canvas.set_draw_color(color);
        self.window_canvas.clear();
    }

    pub fn present_canvas(&mut self) {
        self.window_canvas.present()
    }

    pub fn render_sprite(&mut self, sprite: &SpriteComponent, position: &PositionComponent) {
        let texture = self.textures.get(&sprite.texture_id).expect(&format!(
            "Failed to get texture with ID {}",
            sprite.texture_id
        ));

        let screen_position = Point::new(position.x as i32, position.y as i32);
        let sprite = Rect::new(sprite.x, sprite.y, sprite.w, sprite.h);
        let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
        self.window_canvas
            .copy(texture, sprite, screen_rect)
            .expect("Failed to copy texture")
    }
}
