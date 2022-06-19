use crate::components::position::PositionComponent;
use crate::entity::Entity;
use crate::events::collision::CollisionEvent;
use crate::events::events::Events;

pub enum Collision {
    Rectangle(f32, f32),
    Circle(f32),
}

impl Collision {
    pub fn new_rectangle(w: f32, h: f32) -> Collision {
        Collision::Rectangle(w, h)
    }

    pub fn new_circle(r: f32) -> Collision {
        Collision::Circle(r)
    }

    pub fn process_collisions(
        &self,
        entity_id: usize,
        position: &PositionComponent,
        entities: &Vec<Entity>,
        events: &mut Events,
        world_width: u32,
        world_height: u32,
    ) {
        match self {
            Collision::Rectangle(col_width, col_height) => {
                process_world_collisions(
                    world_width,
                    world_height,
                    *col_width,
                    *col_height,
                    entity_id,
                    position,
                    events,
                );
                process_entities_collisions(
                    *col_width,
                    *col_height,
                    entity_id,
                    position,
                    events,
                    entities,
                );
            }
            Collision::Circle(_) => {
                todo!()
            }
        }
    }
}

fn process_entities_collisions(
    col_width: f32,
    col_height: f32,
    entity_id: usize,
    position: &PositionComponent,
    events: &mut Events,
    entities: &Vec<Entity>,
) {
    entities.iter().for_each(|other_entity| {
        if other_entity.id != entity_id {
            let components = other_entity.components();
            match components.collision.as_ref() {
                Some(Collision::Rectangle(other_col_width, other_col_height)) => {
                    let other_position = components.position.as_ref().unwrap();
                    let mut x_offset = 0.;
                    let mut y_offset = 0.;
                    if position.x - col_width < other_position.x + other_col_width
                        && position.x + col_width > other_position.x - other_col_width
                        && position.y - col_height < other_position.y + other_col_height
                        && position.y + col_height > other_position.y - other_col_height
                    {
                        println!("Collision, offsets: {}, {}", x_offset, y_offset);
                        events.add_collision_event(CollisionEvent {
                            x_offset,
                            y_offset,
                            entity_id,
                            other_entity_id: Some(other_entity.id),
                        })
                    }
                }
                _ => {}
            }
        }
    })
}

fn process_world_collisions(
    world_width: u32,
    world_height: u32,
    col_width: f32,
    col_height: f32,
    entity_id: usize,
    position: &PositionComponent,
    events: &mut Events,
) {
    let mut x_offset = 0.;
    let mut y_offset = 0.;
    if position.x < col_width {
        x_offset = col_width - position.x;
    } else {
        let xx = position.x + col_width - world_width as f32;
        if xx > 0. {
            x_offset -= xx;
        }
    }
    if position.y < col_height {
        y_offset = col_height - position.y;
    } else {
        let yy = position.y + col_height - world_height as f32;
        if yy > 0. {
            y_offset -= yy;
        }
    }
    if x_offset != 0. || y_offset != 0. {
        // println!("{}, {}", x_offset, y_offset);
        events.add_collision_event(CollisionEvent {
            x_offset,
            y_offset,
            entity_id,
            other_entity_id: None,
        })
    }
}
