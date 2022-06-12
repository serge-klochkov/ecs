use std::time::Duration;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;

use ecs::components::{PositionComponent, SpriteComponent, VelocityComponent};
use ecs::entity::{Components, Entity};
use ecs::world::World;

static WIDTH: u32 = 800;
static HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("Failed to init SDL context");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize the video subsystem");
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Failed to initialize SDL2 image subsystem");
    let window = video_subsystem
        .window("ECS", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Failed to create a window");
    let window_canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create a canvas");
    let texture_creator = window_canvas.texture_creator();

    let event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let mut world = World::new(event_pump, window_canvas);

    // Add player entity
    world.add_entity(|id| {
        let sprite_component = SpriteComponent::new(0, Rect::new(0, 0, 64, 64));
        let position_component =
            PositionComponent::new(WIDTH as f32 / 2. - 32., HEIGHT as f32 / 2. - 32.);
        let velocity_component = VelocityComponent::zero();
        Entity::new(
            id,
            "player",
            Components::new(
                Some(sprite_component),
                Some(position_component),
                Some(velocity_component),
            ),
        )
    });
    // 2
    world.add_entity(|id| {
        let sprite_component = SpriteComponent::new(0, Rect::new(0, 0, 64, 64));
        let position_component = PositionComponent::new(100., 100.);
        let velocity_component = VelocityComponent::new(5., 2.);
        Entity::new(
            id,
            "square #2",
            Components::new(
                Some(sprite_component),
                Some(position_component),
                Some(velocity_component),
            ),
        )
    });
    // 3
    world.add_entity(|id| {
        let sprite_component = SpriteComponent::new(0, Rect::new(0, 0, 64, 64));
        let position_component = PositionComponent::new(200., 200.);
        let velocity_component = VelocityComponent::new(2., 2.);
        Entity::new(
            id,
            "square #3",
            Components::new(
                Some(sprite_component),
                Some(position_component),
                Some(velocity_component),
            ),
        )
    });

    let texture = texture_creator
        .load_texture("assets/test.png")
        .expect("Failed to load a texture");
    world.add_texture(0, texture);

    let sleep_duration = Duration::new(0, 1_000_000_000u32 / 60);
    'running: loop {
        world.clear_canvas();
        if !world.process() {
            break 'running;
        }
        world.present_canvas();
        std::thread::sleep(sleep_duration);
    }

    Ok(())
}
