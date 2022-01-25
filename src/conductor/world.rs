use pixelbuffer::{Pixel, PixelBuffer, Resolution, Window};

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
        while self.window.shown() {
            let mut buffer: PixelBuffer = PixelBuffer::new(self.resolution);

            self.window.set_frame(buffer.get_buffer());
        }
    }
}
