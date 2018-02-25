mod gol;

extern crate sdl2;

use gol::world::World;
use gol::gl_renderer::gl_renderer;
use gol::rle_reader::rle_reader;

fn main() {
    let size = 1714;
    let positions = rle_reader::read("./res/tm.lif");
    let world = World::new(positions, size);
    gl_renderer::render(&world);
}
