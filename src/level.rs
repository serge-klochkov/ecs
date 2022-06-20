use crate::components::collision::CollisionComponent;
use crate::components::position::PositionComponent;
use crate::components::rendering::{Border, RenderingComponent};
use crate::entity::{Components, Entity};
use crate::renderer::Renderer;
use crate::world::World;
use sdl2::pixels::Color;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn init_level(world: &mut World, renderer: &mut Renderer) {
    let world_width = world.width();
    let world_height = world.height();
    let tile_size = world.tile_size();
    renderer.add_texture(0, "assets/test.png");
    renderer.add_texture(1, "assets/test2.png");

    // Add player entity
    world.add_entity(|id| {
        let width = 64;
        let height = 64;
        // let rendering_component = RenderingComponent::new_sprite(0, width, height);
        let rendering_component = RenderingComponent::new_rectangle(
            tile_size,
            tile_size,
            // https://lospec.com/palette-list/universalis-42
            Color::from((109, 42, 57)),
            Some(Border {
                border_size: 2,
                border_color: Color::from((206, 161, 67)),
            }),
        );
        let position_component = PositionComponent::new(
            world_width as f32 / 2. - 32.,
            world_height as f32 / 2. - 32.,
            0.,
            0.,
        );
        Entity::new(
            id,
            "player",
            Components::new(
                Some(rendering_component),
                Some(position_component),
                Some(CollisionComponent::Rectangle(width as f32, height as f32)),
            ),
        )
    });
    init_terrain("levels/test.txt", world);
}

fn init_terrain(filename: &str, world: &mut World) {
    let file = File::open(filename).expect(&format!(
        "Failed to open file {}, current directory: {}",
        filename,
        std::env::current_dir().unwrap().display()
    ));
    let tile_size = world.tile_size();
    BufReader::new(file)
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(i, line)| {
            if let Ok(line) = line {
                line.chars().enumerate().for_each(|(j, c)| match c {
                    'x' => {
                        world.add_entity(|id| {
                            // let rendering_component = RenderingComponent::new_sprite(0, tile_size, tile_size);
                            let rendering_component = RenderingComponent::new_rectangle(
                                tile_size,
                                tile_size,
                                Color::from((29, 90, 66)),
                                None,
                                // Some(Border {
                                //     border_size: 2,
                                //     border_color: Color::from((75, 31, 48)),
                                // }),
                            );
                            let position_component = PositionComponent::new(
                                j as f32 * tile_size as f32,
                                i as f32 * tile_size as f32,
                                0.,
                                0.,
                            );
                            Entity::new(
                                id,
                                &format!("Tile {}.{}", i, j),
                                Components::new(
                                    Some(rendering_component),
                                    Some(position_component),
                                    Some(CollisionComponent::Rectangle(
                                        tile_size as f32,
                                        tile_size as f32,
                                    )),
                                ),
                            )
                        });
                    }
                    _ => {}
                });
            }
        })
}
