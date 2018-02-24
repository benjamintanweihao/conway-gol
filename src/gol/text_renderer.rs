#[allow(dead_code)]
pub mod text_renderer {
    use gol::world::{World, SIZE};

    pub fn render(world: &World) -> () {
        print!("{}[2J", 27 as char);
        for y in 0..SIZE {
            for x in 0..SIZE {
                if world.is_alive((x, y)) == true {
                    print!("■");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}
