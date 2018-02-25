#[allow(dead_code)]
pub mod text_renderer {
    use std::{thread, time};
    use gol::world::World;

    pub fn render(world: &World) -> () {
        let mut world = world.tick();
        let duration = time::Duration::from_millis(100);
        let size = world.size;

        loop {
            for y in 0..size {
                for x in 0..size {
                    if world.is_alive((x, y)) == true {
                        print!("■");
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }

            print!("{}[2J", 27 as char);
            thread::sleep(duration);

            world = world.tick();
        }
    }
}
