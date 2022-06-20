use crate::components::position::PositionComponent;
use crate::entity::Entity;
use crate::events::collision::CollisionEvent;
use crate::events::events::Events;

pub enum CollisionComponent {
    Rectangle(f32, f32),
    Circle(f32),
}

impl CollisionComponent {
    pub fn new_rectangle(w: f32, h: f32) -> CollisionComponent {
        CollisionComponent::Rectangle(w, h)
    }

    pub fn new_circle(r: f32) -> CollisionComponent {
        CollisionComponent::Circle(r)
    }

    pub fn process_collisions(
        &self,
        entity_id: usize,
        position: &PositionComponent,
        entities: &Vec<Entity>,
        events: &mut Events,
        _world_width: u32,
        _world_height: u32,
    ) {
        match self {
            CollisionComponent::Rectangle(col_width, col_height) => {
                // process_world_collisions(
                //     world_width,
                //     world_height,
                //     *col_width,
                //     *col_height,
                //     entity_id,
                //     position,
                //     events,
                // );
                process_entities_collisions(
                    *col_width,
                    *col_height,
                    entity_id,
                    position,
                    events,
                    entities,
                );
            }
            CollisionComponent::Circle(_) => {
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
                Some(CollisionComponent::Rectangle(other_col_width, other_col_height)) => {
                    let other_position = components.position.as_ref().unwrap();
                    let (collision_time, normal_x, normal_y) = swept_aabb(
                        (
                            position.x,
                            position.y,
                            position.vx,
                            position.vy,
                            col_width,
                            col_height,
                        ),
                        (
                            other_position.x,
                            other_position.y,
                            *other_col_width,
                            *other_col_height,
                        ),
                    );
                    if collision_time < 1. {
                        let remaining_time = 1. - collision_time;
                        println!("{} {} {}", remaining_time, normal_x, normal_y);
                        let dot_product =
                            (position.vx * normal_y + position.vy * normal_x) * remaining_time;

                        let new_x = position.x + position.vx * collision_time;
                        let new_y = position.y + position.vy * collision_time;
                        let new_vx = dot_product * normal_y;
                        let new_vy = dot_product * normal_x;
                        events.add_collision_event(CollisionEvent {
                            entity_id,
                            new_x,
                            new_y,
                            new_vx,
                            new_vy,
                            other_entity_id: Some(other_entity.id),
                        })
                    }
                }
                _ => {}
            }
        }
    })
}

// fn process_world_collisions(
//     world_width: u32,
//     world_height: u32,
//     col_width: f32,
//     col_height: f32,
//     entity_id: usize,
//     position: &PositionComponent,
//     events: &mut Events,
// ) {
//     let mut x_offset = 0.;
//     let mut y_offset = 0.;
//     if position.x < 0. {
//         x_offset = -position.x;
//     } else {
//         let xx = position.x + col_width - world_width as f32;
//         if xx > 0. {
//             x_offset -= xx;
//         }
//     }
//     if position.y < 0. {
//         y_offset = -position.y;
//     } else {
//         let yy = position.y + col_height - world_height as f32;
//         if yy > 0. {
//             y_offset -= yy;
//         }
//     }
//     if x_offset != 0. || y_offset != 0. {
//         // println!("{}, {}", x_offset, y_offset);
//         events.add_collision_event(CollisionEvent {
//             entity_id,
//             x_offset: Some(x_offset),
//             y_offset: Some(y_offset),
//             new_vx: None,
//             new_vy: None,
//             other_entity_id: None,
//         })
//     }
// }

/// See https://www.gamedev.net/tutorials/programming/general-and-gameplay-programming/swept-aabb-collision-detection-and-response-r3084/
///
/// Returns (collision_time, normal_x, normal_y)
fn swept_aabb(
    (x1, y1, vx1, vy1, w1, h1): (f32, f32, f32, f32, f32, f32),
    (x2, y2, w2, h2): (f32, f32, f32, f32),
) -> (f32, f32, f32) {
    let (x_close, y_close): (f32, f32);
    let (x_far, y_far): (f32, f32);
    let (x_entry, y_entry): (f32, f32);
    let (x_exit, y_exit): (f32, f32);

    if vx1 > 0. {
        x_close = x2 - (x1 + w1);
        x_far = (x2 + w2) - x1;
    } else {
        x_close = (x2 + w2) - x1;
        x_far = x2 - (x1 + w1);
    }

    if vy1 > 0. {
        y_close = y2 - (y1 + h1);
        y_far = (y2 + h2) - y1;
    } else {
        y_close = (y2 + h2) - y1;
        y_far = y2 - (y1 + h1);
    }

    if vx1 == 0. {
        x_entry = f32::NEG_INFINITY;
        x_exit = f32::INFINITY;
    } else {
        x_entry = x_close / vx1;
        x_exit = x_far / vx1;
    }

    if vy1 == 0. {
        y_entry = f32::NEG_INFINITY;
        y_exit = f32::INFINITY;
    } else {
        y_entry = y_close / vy1;
        y_exit = y_far / vy1;
    }

    let entry_time = f32::max(x_entry, y_entry);
    let exit_time = f32::min(x_exit, y_exit);

    return if entry_time > exit_time
        || (x_entry < 0. && y_entry < 0.)
        || x_entry > 1.
        || y_entry > 1.
    {
        (1.0, 0., 0.)
    } else {
        if x_entry > y_entry {
            if x_close < 0. {
                (entry_time, 1.0, 0.)
            } else {
                (entry_time, -1.0, 0.)
            }
        } else {
            if y_close < 0. {
                (entry_time, 0., 1.0)
            } else {
                (entry_time, 0., -1.0)
            }
        }
    };
}
