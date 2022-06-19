use crate::entity::Entity;
use crate::events::collision::CollisionEvent;
use std::collections::HashSet;

pub struct Events {
    collision: Vec<CollisionEvent>,
    collisions_to_skip: HashSet<usize>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            collision: vec![],
            collisions_to_skip: HashSet::new(),
        }
    }

    pub fn add_collision_event(&mut self, collision_event: CollisionEvent) {
        self.collision.push(collision_event)
    }

    pub fn process(&mut self, entities: &Vec<Entity>) {
        self.collision.iter().for_each(
            |CollisionEvent {
                 x_offset,
                 y_offset,
                 entity_id,
                 other_entity_id,
             }| {
                if !self.collisions_to_skip.contains(entity_id) {
                    println!("Processing entity {}", entity_id);
                    let entity = entities
                        .get(*entity_id)
                        .expect(&format!("Could not get entity with ID {}", entity_id));
                    let mut components = entity.components_mut();
                    let mut position = components.position.as_mut().unwrap();
                    position.x += x_offset;
                    position.y += y_offset;
                    match other_entity_id {
                        Some(id) => {
                            self.collisions_to_skip.insert(*id);
                        }
                        _ => {}
                    }
                }
            },
        );
        self.collision.clear();
        self.collisions_to_skip.clear();
    }
}
