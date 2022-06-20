use crate::entity::Entity;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::collections::HashSet;

pub struct Controls {
    event_pump: EventPump,
    is_paused: bool,
}

static MAX_VELOCITY: f32 = 5.;
static ACCELERATION: f32 = 1.;

impl Controls {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            is_paused: false,
        }
    }

    pub fn handle_player_controls(&mut self, player_entity: &Entity) -> bool {
        self.event_pump.pump_events();
        let pressed_keycodes = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect::<HashSet<Keycode>>();
        if pressed_keycodes.contains(&Keycode::Escape) {
            return false;
        }
        let mut components = player_entity.components_mut();
        let mut position = components.position.as_mut().unwrap();
        if self.is_paused && !pressed_keycodes.is_empty() {
            self.is_paused = false;
        }
        if pressed_keycodes.contains(&Keycode::Space) {
            position.vx = 0.;
            position.vy = 0.;
            self.is_paused = true;
        }
        if pressed_keycodes.contains(&Keycode::Up) {
            let new_velocity = position.vy - ACCELERATION;
            position.vy = f32::max(-MAX_VELOCITY, new_velocity);
        } else if !self.is_paused {
            let new_velocity = position.vy + ACCELERATION;
            position.vy = f32::min(MAX_VELOCITY, new_velocity);
        }
        if pressed_keycodes.contains(&Keycode::Left) {
            let new_velocity = position.vx - ACCELERATION;
            position.vx = f32::max(-MAX_VELOCITY, new_velocity);
        }
        if pressed_keycodes.contains(&Keycode::Right) {
            let new_velocity = position.vx + ACCELERATION;
            position.vx = f32::min(MAX_VELOCITY, new_velocity);
        }
        true
    }
}
