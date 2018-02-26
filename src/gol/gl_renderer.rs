#[allow(dead_code)]
pub mod gl_renderer {
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use std::{thread, time};
    use gol::world::World;

    pub fn render(world: &World) -> () {
        let (cell_size, draw_cell_size) = match world.size {
            0...400 => (4, 3),
            401...500 => (2, 2),
            _ => (1, 1),
        };

        let sdl_context = ::sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let size: u32 = world.size as u32 * cell_size;
        let window = video_subsystem
            .window("Conway's Game of Life", size, size)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let mut event_pump = sdl_context.event_pump().unwrap();
        let duration = time::Duration::from_millis(10);

        let mut world = world.tick();
        let size = world.size;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            let cell_color = Color::RGB(0, 255, 0);
            canvas.set_draw_color(cell_color);

            for y in 0..size {
                for x in 0..size {
                    if world.is_alive((x, y)) == true {
                        let _ = canvas.fill_rect(Rect::new(
                            (x as i32) * (cell_size as i32),
                            (y as i32) * (cell_size as i32),
                            draw_cell_size,
                            draw_cell_size,
                        ));
                    }
                }
            }

            canvas.present();

            thread::sleep(duration);
            world = world.tick();
        }
    }
}
