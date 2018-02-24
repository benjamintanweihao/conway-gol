#[allow(dead_code)]
pub mod text_renderer {
    use std::{thread, time};
    use gol::world::{World, SIZE};

    pub fn render(world: World) -> () {
        let mut world = world;
        let duration = time::Duration::from_millis(100);

        loop {
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

            print!("{}[2J", 27 as char);
            thread::sleep(duration);

            world = world.tick();
        }
    }
}
