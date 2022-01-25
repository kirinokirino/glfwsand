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

mod gfx;
use gfx::{Circle, PixelBuffer};

mod ufb;
use ufb::{Resolution, Window};

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
