use std::time::Duration;

use ecs::controls::Controls;
use ecs::events::events::Events;

use ecs::level::init_level;
use ecs::renderer::Renderer;
use ecs::world::World;

static TILE_SIZE: u32 = 64;
static WIDTH_TILES: u32 = 24;
static HEIGHT_TILES: u32 = 16;

fn main() -> Result<(), String> {
    let context = sdl2::init().expect("Failed to init SDL context");
    let event_pump = context.event_pump().expect("Failed to get event pump");

    let mut events = Events::new();
    let mut world = World::new(WIDTH_TILES, HEIGHT_TILES, TILE_SIZE);
    let mut renderer = Renderer::new(&context, world.width(), world.height());
    let mut controls = Controls::new(event_pump);
    init_level(&mut world, &mut renderer);

    let sleep_duration = Duration::new(0, 1_000_000_000u32 / 60);
    'running: loop {
        let mut player_entity = world.get_player_entity();
        if !controls.handle_player_controls(&mut player_entity) {
            break 'running;
        }
        renderer.clear_canvas();
        world.update(&mut events);
        world.each_entity(|e| {
            let components = e.components();
            match components.rendering.as_ref() {
                Some(rendering) => {
                    let position = components.position.as_ref().unwrap();
                    renderer.render(rendering, position)
                }
                _ => {}
            }
        });
        renderer.present_canvas();
        std::thread::sleep(sleep_duration);
    }

    Ok(())
}
