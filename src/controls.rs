use crate::entity::Entity;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Controls {
    event_pump: EventPump,
}

static DEFAULT_VELOCITY: f32 = 10.;

impl Controls {
    pub fn new(event_pump: EventPump) -> Self {
        Self { event_pump }
    }

    pub fn handle_player_controls(&mut self, player_entity: &Entity) -> bool {
        for event in self.event_pump.poll_iter() {
            let mut components = player_entity.components_mut();
            let mut position = components.position.as_mut().unwrap();
            match event {
                // Game state
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                // Movement
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => position.velocity.x = -DEFAULT_VELOCITY,
                Event::KeyUp {
                    keycode: Some(Keycode::Left | Keycode::Right),
                    ..
                } => position.velocity.x = 0.,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => position.velocity.x = DEFAULT_VELOCITY,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => position.velocity.y = -DEFAULT_VELOCITY,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => position.velocity.y = DEFAULT_VELOCITY,
                Event::KeyUp {
                    keycode: Some(Keycode::Up | Keycode::Down),
                    ..
                } => position.velocity.y = 0.,
                _ => {}
            }
        }
        true
    }
}
