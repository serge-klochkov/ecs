use crate::components::*;

pub struct Entity {
    pub id: usize,
    pub name: String,
    pub components: Components,
}

impl Entity {
    pub fn new(id: usize, name: &str, components: Components) -> Self {
        Self {
            id,
            components,
            name: name.to_string(),
        }
    }

    pub fn borrow_velocity_mut(&mut self) -> &mut VelocityComponent {
        self.components
            .velocity
            .as_mut()
            .expect("Failed to obtain velocity component reference")
    }
}

pub struct Components {
    pub sprite: Option<SpriteComponent>,
    pub position: Option<PositionComponent>,
    pub velocity: Option<VelocityComponent>,
}

impl Components {
    pub fn new(
        sprite: Option<SpriteComponent>,
        position: Option<PositionComponent>,
        velocity: Option<VelocityComponent>,
    ) -> Self {
        Self {
            sprite,
            position,
            velocity,
        }
    }

    pub fn empty() -> Self {
        Components {
            sprite: None,
            position: None,
            velocity: None,
        }
    }
}
