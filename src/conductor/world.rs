use pixelbuffer::{Event, Pixel, PixelBuffer, Resolution, Window};

pub struct World {
    window: Window,
    resolution: Resolution,
}

impl World {
    pub fn new(resolution: Resolution, title: &str) -> Self {
        Self {
            window: Window::new(resolution, title),
            resolution,
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
                        _ => println!("Pressed unhandled key {:?}", key),
                    },
                }
            }
            let mut buffer: PixelBuffer = PixelBuffer::new(self.resolution);

            self.window.set_frame(buffer.get_buffer());
        }
    }
}
