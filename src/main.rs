#![warn(
    clippy::pedantic,
    clippy::nursery,
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
    clippy::verbose_file_reads
)]
mod ufb;
use ufb::{ColorDepth, Window};

const WIDTH: u16 = 640;
const HEIGHT: u16 = 640;

fn main() {
    let mut win = Window::new(WIDTH, HEIGHT, ColorDepth::Rgb8, "game");
    for (iter, pixel) in win.get_frame().chunks_exact_mut(3).enumerate() {
        let x = iter % WIDTH as usize;
        let y = iter / WIDTH as usize;
        let val = x ^ y;
        let hex = format!("{:06x}", val);
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        pixel.copy_from_slice(&[r, g, b]);
    }
    win.show();
}
