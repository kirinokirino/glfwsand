#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::unwrap_in_result,
    clippy::unneeded_field_pattern,
    clippy::string_to_string,
    clippy::string_slice,
    clippy::string_add,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::rc_mutex,
    clippy::rc_buffer,
    clippy::pattern_type_mismatch,
    clippy::multiple_inherent_impl,
    clippy::missing_enforced_import_renames,
    clippy::lossy_float_literal,
    clippy::let_underscore_must_use,
    clippy::integer_division,
    clippy::inline_asm_x86_att_syntax,
    clippy::indexing_slicing,
    clippy::if_then_some_else_none,
    clippy::get_unwrap,
    clippy::fn_to_numeric_cast,
    clippy::float_cmp_const,
    clippy::filetype_is_file,
    clippy::create_dir,
    clippy::clone_on_ref_ptr,
    clippy::as_conversions,
    clippy::verbose_file_reads,
    clippy::missing_safety_doc
)]

use glam::Vec2;

use pixelbuffer::{Pixel, PixelBuffer, Resolution, Window};

const WIDTH: u16 = 640;
const HEIGHT: u16 = 640;
const RESOLUTION: Resolution = Resolution::new(WIDTH, HEIGHT);

fn main() {
    let mut window = Window::new(RESOLUTION, "game");
    let mut counter = 50.0;
    while window.shown() {
        counter += 1.0;
        let mut buffer: PixelBuffer = PixelBuffer::new(RESOLUTION);
        let circle = Circle::new(Vec2::new(counter, 50.), 20.0, 100, 255);
        circle.draw(&mut buffer);
        window.set_frame(buffer.get_buffer());
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
