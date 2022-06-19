use crate::entity::*;
use crate::events::events::Events;

pub struct World {
    width: u32,
    height: u32,
    entities: Vec<Entity>,
    events: Events,
    last_id: usize,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        World {
            entities: vec![],
            events: Events::new(),
            last_id: 0,
            width,
            height,
        }
    }

    pub fn get_entity_by_id(&mut self, id: usize) -> &mut Entity {
        self.entities
            .get_mut(id)
            .expect(&format!("Entity with ID {} not found", id))
    }

    pub fn get_player_entity(&mut self) -> &mut Entity {
        self.get_entity_by_id(0)
    }

    pub fn add_entity<F>(&mut self, new_entity: F) -> ()
    where
        F: Fn(usize) -> Entity,
    {
        let entity = new_entity(self.last_id);
        self.last_id += 1;
        self.entities.push(entity);
    }

    pub fn update(&mut self) -> bool {
        for i in 0..self.entities.len() {
            let entity = &self.entities[i];
            let mut components = entity.components_mut();
            match components.position.as_mut() {
                Some(position) => position.apply_velocity(),
                None => {}
            }
            match components.collision.as_ref() {
                None => {}
                Some(collision) => {
                    // let components = entity.components.borrow();
                    let position = components.position.as_ref().unwrap();
                    collision.process_collisions(
                        entity.id,
                        position,
                        &self.entities,
                        &mut self.events,
                        self.width,
                        self.height,
                    );
                }
            }
        }
        self.events.process(&self.entities);
        true
    }

    pub fn each_entity<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Entity) -> (),
    {
        self.entities.iter_mut().for_each(|entity| f(entity))
    }
}
