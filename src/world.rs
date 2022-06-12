use crate::controls::handle_player_controls;
use crate::entity::*;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;
use std::collections::HashMap;

pub struct World<'a> {
    entities: Vec<Entity>,
    textures: HashMap<usize, Texture<'a>>,
    last_id: usize,
    window_canvas: WindowCanvas,
    event_pump: EventPump,
}

impl<'a> World<'a> {
    pub fn new(event_pump: EventPump, window_canvas: WindowCanvas) -> Self {
        World {
            entities: vec![],
            textures: HashMap::new(),
            last_id: 0,
            window_canvas,
            event_pump,
        }
    }

    pub fn add_texture(&mut self, texture_id: usize, texture: Texture<'a>) {
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

    pub fn get_entity_by_id(&mut self, id: usize) -> &mut Entity {
        self.entities
            .get_mut(id)
            .expect(&format!("Entity with ID {} not found", id))
    }

    pub fn add_entity<F>(&mut self, new_entity: F) -> ()
    where
        F: Fn(usize) -> Entity,
    {
        let entity = new_entity(self.last_id);
        self.last_id += 1;
        self.entities.push(entity);
    }

    pub fn process(&mut self) -> bool {
        let mut player_entity = self
            .entities
            .get_mut(0)
            .expect("Could not get player entity");
        if !handle_player_controls(&mut self.event_pump, &mut player_entity) {
            return false;
        }
        for i in 0..self.entities.len() {
            let entity = &mut self.entities[i];
            match entity.components.velocity.as_mut() {
                None => {}
                Some(velocity) => {
                    // has velocity == has position
                    let position = entity.components.position.as_mut().unwrap();
                    position.update(&velocity);
                }
            }
            match entity.components.sprite.as_mut() {
                None => {}
                Some(sprite) => {
                    // has sprite == has position
                    let position = entity.components.position.as_ref().unwrap();
                    sprite.render(&mut self.window_canvas, position, &self.textures);
                }
            }
        }
        true
    }
}
