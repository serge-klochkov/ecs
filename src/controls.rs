use crate::entity::Entity;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub fn handle_player_controls(event_pump: &mut EventPump, player_entity: &mut Entity) -> bool {
    for event in event_pump.poll_iter() {
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
            } => player_entity.borrow_velocity_mut().x = -3.,
            Event::KeyUp {
                keycode: Some(Keycode::Left | Keycode::Right),
                ..
            } => player_entity.borrow_velocity_mut().x = 0.,
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => player_entity.borrow_velocity_mut().x = 3.,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => player_entity.borrow_velocity_mut().y = -3.,
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => player_entity.borrow_velocity_mut().y = 3.,
            Event::KeyUp {
                keycode: Some(Keycode::Up | Keycode::Down),
                ..
            } => player_entity.borrow_velocity_mut().y = 0.,
            _ => {}
        }
    }
    true
}
