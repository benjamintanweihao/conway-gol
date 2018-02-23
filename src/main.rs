mod gol;

use std::{thread, time};
use gol::world::World;
use gol::text_renderer::text_renderer;

fn main() {
    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let combination = vec![
        (0, 12),
        (1, 12),
        (2, 12),
        (1, 6),
        (2, 7),
        (0, 8),
        (1, 8),
        (2, 8),
    ];

    let world = World::new(combination);

    let duration = time::Duration::from_millis(100);
    let mut world = world;
    loop {
        text_renderer::render(&world);
        world = world.tick();
        thread::sleep(duration);
    }
}
