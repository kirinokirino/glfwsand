use crate::automata::random_walker::Walker;
use crate::common::Position;
use pixelbuffer::{Event, Pixel, PixelBuffer, Resolution, Window};
pub struct World {
    window: Window,
    resolution: Resolution,

    walkers: Vec<Walker>,
}

impl World {
    pub fn new(resolution: Resolution, title: &str) -> Self {
        Self {
            window: Window::new(resolution, title),
            resolution,
            walkers: Vec::new(),
        }
    }
    pub fn add_walkers(&mut self) {
        for _ in 0..50 {
            self.walkers.push(Walker::new(Position::new(
                fastrand::i64(50..400),
                fastrand::i64(50..400),
            )));
        }
    }

    pub fn start(&mut self) {
        loop {
            if let Some(event) = self.window.shown() {
                match event {
                    Event::Close => break,
                    Event::Key(key) => match key {
                        glfw::Key::W => (),
                        glfw::Key::S => (),
                        glfw::Key::A => (),
                        glfw::Key::D => (),
                        glfw::Key::Space => self.add_walkers(),
                        _ => println!("Pressed unhandled key {:?}", key),
                    },
                }
            }
            let mut buffer: PixelBuffer = PixelBuffer::new(self.resolution);
            for walker in &mut self.walkers {
                walker.update();
            }
            for walker in &self.walkers {
                let x = walker.position.x;
                let y = walker.position.y;
                buffer.set_pixel((x as u16, y as u16).into(), Pixel::new(255, 255, 255));
            }
            self.window.set_frame(buffer.get_buffer());
        }
    }
}
