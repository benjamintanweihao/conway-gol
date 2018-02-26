mod gol;

extern crate sdl2;

use gol::world::World;
use gol::gl_renderer::gl_renderer;
use gol::rle_reader::rle_reader;

fn main() {
    let size = 664;
    let positions = rle_reader::read("./res/cordership.rle");
    let world = World::new(positions, size);
    gl_renderer::render(&world);
}
