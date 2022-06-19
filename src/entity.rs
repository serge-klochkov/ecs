use crate::components::collision::Collision;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use std::cell::{Ref, RefCell, RefMut};

pub struct Entity {
    pub id: usize,
    pub name: String,
    components: RefCell<Components>,
}

impl Entity {
    pub fn new(id: usize, name: &str, components: RefCell<Components>) -> Self {
        Self {
            id,
            components,
            name: name.to_string(),
        }
    }

    pub fn components(&self) -> Ref<Components> {
        self.components.borrow()
    }

    pub fn components_mut(&self) -> RefMut<'_, Components> {
        self.components.borrow_mut()
    }

    // pub fn borrow_velocity_mut(&mut self) -> &mut Velocity {
    //     &mut self
    //         .components
    //         .borrow_mut()
    //         .position
    //         .as_mut()
    //         .expect("Failed to obtain position component reference")
    //         .velocity
    // }
}

pub struct Components {
    pub sprite: Option<SpriteComponent>,
    pub position: Option<PositionComponent>,
    pub collision: Option<Collision>,
}

impl Components {
    pub fn new(
        sprite: Option<SpriteComponent>,
        position: Option<PositionComponent>,
        collision: Option<Collision>,
    ) -> RefCell<Self> {
        RefCell::new(Self {
            sprite,
            position,
            collision,
        })
    }
}
