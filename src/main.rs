use ecs::components::collision::Collision;
use std::time::Duration;

use ecs::components::position::PositionComponent;
use ecs::components::sprite::SpriteComponent;

use ecs::entity::{Components, Entity};
use ecs::renderer::Renderer;
use ecs::world::World;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);
    let mut controls = renderer.init_controls();
    let mut world = World::new(WIDTH, HEIGHT);

    renderer.add_texture(0, "assets/test.png");

    // Add player entity
    world.add_entity(|id| {
        let width = 64;
        let height = 64;
        let sprite_component = SpriteComponent::new(0, 0, 0, width, height);
        let position_component =
            PositionComponent::new(WIDTH as f32 / 2. - 32., HEIGHT as f32 / 2. - 32., 0., 0.);
        Entity::new(
            id,
            "player",
            Components::new(
                Some(sprite_component),
                Some(position_component),
                Some(Collision::Rectangle(width as f32 / 2., height as f32 / 2.)),
            ),
        )
    });
    // 2
    world.add_entity(|id| {
        let width = 64;
        let height = 64;
        let sprite_component = SpriteComponent::new(0, 0, 0, width, height);
        let position_component = PositionComponent::new(100., 100., 0., 0.);
        Entity::new(
            id,
            "square #2",
            Components::new(
                Some(sprite_component),
                Some(position_component),
                Some(Collision::Rectangle(width as f32 / 2., height as f32 / 2.)),
            ),
        )
    });
    // 3
    world.add_entity(|id| {
        let width = 64;
        let height = 64;
        let sprite_component = SpriteComponent::new(0, 0, 0, height, width);
        let position_component = PositionComponent::new(200., 200., 0., 0.);
        Entity::new(
            id,
            "square #3",
            Components::new(
                Some(sprite_component),
                Some(position_component),
                Some(Collision::Rectangle(width as f32 / 2., height as f32 / 2.)),
            ),
        )
    });

    let sleep_duration = Duration::new(0, 1_000_000_000u32 / 60);
    'running: loop {
        let mut player_entity = world.get_player_entity();
        if !controls.handle_player_controls(&mut player_entity) {
            break 'running;
        }
        renderer.clear_canvas();
        world.update();
        world.each_entity(|e| {
            let mut components = e.components();
            match components.sprite.as_ref() {
                Some(sprite) => {
                    let position = components.position.as_ref().unwrap();
                    renderer.render_sprite(sprite, position)
                }
                _ => {}
            }
        });
        renderer.present_canvas();
        std::thread::sleep(sleep_duration);
    }

    Ok(())
}
