use std::collections::HashMap;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::Sdl;

use crate::components::position::PositionComponent;
use crate::components::rendering::{Border, Rectangle, RenderingComponent, Sprite};

pub struct Renderer {
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<usize, Texture>,
}

impl Renderer {
    pub fn new(context: &Sdl, width: u32, height: u32) -> Self {
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
        let canvas = window
            .into_canvas()
            .build()
            .expect("Failed to create a canvas");
        let texture_creator = canvas.texture_creator();
        Self {
            textures: HashMap::new(),
            texture_creator,
            canvas,
        }
    }

    pub fn add_texture(&mut self, texture_id: usize, path: &str) {
        let texture: Texture = self
            .texture_creator
            .load_texture(path)
            .expect(&format!("Failed to load a texture {}", path));
        self.textures.insert(texture_id, texture);
    }

    pub fn clear_canvas(&mut self) {
        let color = Color::from((87, 148, 199));
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present_canvas(&mut self) {
        self.canvas.present()
    }

    pub fn render(
        &mut self,
        rendering_component: &RenderingComponent,
        position: &PositionComponent,
    ) {
        match rendering_component {
            RenderingComponent::Sprite(sprite) => self.render_sprite(sprite, position),
            RenderingComponent::Rectangle(rectangle) => self.render_rectangle(rectangle, position),
        }
    }

    pub fn render_rectangle(&mut self, rectangle: &Rectangle, position: &PositionComponent) {
        self.canvas.set_draw_color(rectangle.color);
        self.canvas
            .fill_rect(Rect::new(
                position.x as i32,
                position.y as i32,
                rectangle.w,
                rectangle.h,
            ))
            .unwrap();
        match rectangle.border {
            None => {}
            Some(Border {
                border_size,
                border_color,
            }) => {
                self.canvas.set_draw_color(border_color);
                for i in 0..border_size {
                    self.canvas
                        .draw_rect(Rect::new(
                            position.x as i32 + i as i32,
                            position.y as i32 + i as i32,
                            rectangle.w - (i * 2),
                            rectangle.h - (i * 2),
                        ))
                        .unwrap();
                }
            }
        }
    }

    pub fn render_sprite(&mut self, sprite: &Sprite, position: &PositionComponent) {
        let texture = self.textures.get(&sprite.texture_id).expect(&format!(
            "Failed to get texture with ID {}",
            sprite.texture_id
        ));
        let sprite_rectangle = Rect::new(0, 0, sprite.w, sprite.h);
        let screen_rect = Rect::new(position.x as i32, position.y as i32, sprite.w, sprite.h);
        self.canvas
            .copy(texture, sprite_rectangle, screen_rect)
            .expect("Failed to copy texture")
    }
}
