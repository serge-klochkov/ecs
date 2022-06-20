use crate::components::collision::CollisionComponent;
use crate::components::position::PositionComponent;
use crate::components::rendering::RenderingComponent;
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
}

pub struct Components {
    pub rendering: Option<RenderingComponent>,
    pub position: Option<PositionComponent>,
    pub collision: Option<CollisionComponent>,
}

impl Components {
    pub fn new(
        rendering: Option<RenderingComponent>,
        position: Option<PositionComponent>,
        collision: Option<CollisionComponent>,
    ) -> RefCell<Self> {
        RefCell::new(Self {
            rendering,
            position,
            collision,
        })
    }
}
