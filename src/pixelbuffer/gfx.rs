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
}
impl Default for Pixel {
    fn default() -> Self {
        Self::alpha()
    }
}

pub struct PixelBuffer {
    resolution: Resolution,
    buffer: Vec<Pixel>,
}

impl PixelBuffer {
    #[must_use]
    pub fn new(resolution: Resolution) -> Self {
        Self {
            resolution,
            buffer: vec![Pixel::default(); resolution.area()],
        }
    }

    pub fn set_pixel(&mut self, coords: FramebufferCoordinates, mut pixel: Pixel) {
        if pixel == Pixel::alpha() {
            return;
        }
        let (x, y) = coords.into();
        let pixel_position = self
            .buffer
            .get_mut(usize::from(x) + usize::from(y) * usize::from(self.resolution.width));
        match pixel_position {
            Some(pixel_position) => std::mem::swap(pixel_position, &mut pixel),
            None => println!("tried to set pixel outside of pixelbuffer."),
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
