use crate::ufb::{FramebufferCoordinates, Resolution};
use glam::Vec2;
use std::default::Default;

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
impl Default for Pixel {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

pub struct PixelBuffer {
    resolution: Resolution,
    buffer: Vec<Pixel>,
}

impl PixelBuffer {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            resolution,
            buffer: vec![Pixel::default(); resolution.area()],
        }
    }

    pub fn set_pixel(&mut self, coords: FramebufferCoordinates, pixel: Pixel) {
        let (x, y) = coords.into();
        self.buffer[usize::from(x) + usize::from(y) * usize::from(self.resolution.width)] = pixel;
    }

    pub fn get_buffer(self) -> Vec<Pixel> {
        self.buffer
    }
}

#[derive(Clone)]
pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
    pub color_center: u32,
    pub color_end: u32,
}

impl Circle {
    pub fn new(origin: Vec2, radius: f32, color_center: u32, color_end: u32) -> Self {
        Self {
            origin,
            radius,
            color_center,
            color_end,
        }
    }

    pub fn draw(&self, buffer: &mut PixelBuffer) {
        for screen_y in 0..200 {
            for screen_x in 0..200 {
                let screen_point = Vec2::new(screen_x as f32, screen_y as f32);
                let distance = self.origin.distance(screen_point);
                if distance > self.radius {
                    continue;
                }
                let color = map(
                    distance,
                    0.0,
                    self.radius,
                    self.color_center as f32,
                    self.color_end as f32,
                );
                let color = color as u8;
                let pixel = Pixel::new(color, color, color);
                buffer.set_pixel((screen_x, screen_y).into(), pixel);
            }
        }
    }
}

fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    (value - start1) / (stop1 - start1) * (stop2 - start2) + start2
}
