use crate::entity::Entity;
use crate::events::collision::CollisionEvent;

pub struct Events {
    collision: Vec<CollisionEvent>,
}

impl Events {
    pub fn new() -> Self {
        Self { collision: vec![] }
    }

    pub fn add_collision_event(&mut self, collision_event: CollisionEvent) {
        self.collision.push(collision_event)
    }

    pub fn process(&mut self, entities: &Vec<Entity>) {
        self.collision.iter().for_each(
            |CollisionEvent {
                 new_x,
                 new_y,
                 new_vx,
                 new_vy,
                 entity_id,
                 ..
             }| {
                let entity = get_entity(entities, *entity_id);
                let mut components = entity.components_mut();
                let position = components.position.as_mut().unwrap();
                position.x = *new_x;
                position.y = *new_y;
                position.vx = *new_vx;
                position.vy = *new_vy;
            },
        );
        self.collision.clear();
    }
}

fn get_entity(entities: &Vec<Entity>, entity_id: usize) -> &Entity {
    entities
        .get(entity_id)
        .expect(&format!("Could not get entity with ID {}", entity_id))
}
