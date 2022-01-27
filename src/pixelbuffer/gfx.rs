use crate::ufb::Resolution;
use std::default::Default;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub const fn alpha() -> Self {
        Self::new(0xff, 0, 0xba)
    }

    #[must_use]
    pub const fn black() -> Self {
        Self::new(0, 0, 0)
    }

    #[must_use]
    pub const fn white() -> Self {
        Self::new(255, 255, 255)
    }
}
impl Default for Pixel {
    fn default() -> Self {
        Self::alpha()
    }
}

pub struct PixelBuffer {
    bounds: Resolution,
    buffer: Vec<Pixel>,
}

impl PixelBuffer {
    #[must_use]
    pub fn new(resolution: Resolution) -> Self {
        Self {
            bounds: resolution,
            buffer: vec![Pixel::black(); resolution.area()],
        }
    }

    pub fn set_pixel(&mut self, coords: FramebufferCoordinates, mut pixel: Pixel) {
        if pixel == Pixel::alpha() {
            return;
        }
        let (x, y) = coords.into();
        let pixel_position = self
            .buffer
            .get_mut(usize::from(x) + usize::from(y) * usize::from(self.bounds.width));
        match pixel_position {
            Some(pixel_position) => std::mem::swap(pixel_position, &mut pixel),
            None => println!("tried to set pixel outside of pixelbuffer."),
        }
    }

    pub fn blit(
        &mut self,
        top_left: FramebufferCoordinates,
        sprite_size: Resolution,
        sprite: &mut [Pixel],
    ) {
        let (offset_x, offset_y) = top_left.into();
        let (sprite_width, sprite_height) = sprite_size.into();
        let buffer_right_border = self.bounds.width.min(offset_x + sprite_width);
        let buffer_down_border = self.bounds.height.min(offset_y + sprite_height);

        for buffer_y in offset_y..buffer_down_border {
            for buffer_x in offset_x..buffer_right_border {
                let width = buffer_right_border - offset_x;
                let sprite_index =
                    usize::from((buffer_y - offset_y) * width + (buffer_x - offset_x));
                let sprite_pixel = sprite
                    .get_mut(sprite_index)
                    .expect("Index out of sprite in blit, should not be possible.");

                if sprite_pixel == &Pixel::alpha() {
                    continue;
                }
                let buffer_pixel = self
                    .buffer
                    .get_mut(
                        usize::from(buffer_x)
                            + usize::from(buffer_y) * usize::from(self.bounds.width),
                    )
                    .expect("Index out of buffer in blit, should not be possible.");
                std::mem::swap(buffer_pixel, sprite_pixel);
            }
        }
    }

    #[must_use]
    pub fn get_buffer(self) -> Vec<Pixel> {
        self.buffer
    }
}

#[derive(Clone, Copy)]
pub struct FramebufferCoordinates((u16, u16));

impl From<(u16, u16)> for FramebufferCoordinates {
    fn from(coords: (u16, u16)) -> Self {
        Self(coords)
    }
}

impl From<FramebufferCoordinates> for (u16, u16) {
    fn from(coords: FramebufferCoordinates) -> (u16, u16) {
        (coords.0 .0, coords.0 .1)
    }
}

#[must_use]
pub fn xy(index: usize, resolution: Resolution) -> FramebufferCoordinates {
    let (width, height) = resolution.into();
    debug_assert!(index < usize::from(width) * usize::from(height));
    unsafe {
        // SAFETY: debug assertion that the usize fits into u16.
        let x: u16 = (index % usize::from(width)).try_into().unwrap_unchecked();
        #[allow(clippy::integer_division)]
        let y: u16 = (index / usize::from(width)).try_into().unwrap_unchecked();
        FramebufferCoordinates::from((x, y))
    }
}
