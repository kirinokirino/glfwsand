extern crate glfw;
use glu_sys::glu::{
    glDrawPixels, glPixelZoom, glRasterPos2i, GL_LUMINANCE, GL_LUMINANCE_ALPHA, GL_RGB, GL_RGBA,
    GL_UNSIGNED_BYTE,
};

use glfw::{Action, Context, Key};
use std::convert::{From, Into};

/// Supported color depths
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ColorDepth {
    L8,
    La8,
    Rgb8,
    Rgba8,
}

impl From<ColorDepth> for usize {
    fn from(value: ColorDepth) -> Self {
        match value {
            ColorDepth::L8 => 1,
            ColorDepth::La8 => 2,
            ColorDepth::Rgb8 => 3,
            ColorDepth::Rgba8 => 4,
        }
    }
}

/// Wrapper around a glfw window
pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    pub resolution: Resolution,
    visual: ColorDepth,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    frame: Vec<u8>,
}

impl Window {
    /// Instantiate a window
    pub fn new(w: u16, h: u16, visual: ColorDepth, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Couldn't initialize the window.");
        let (mut window, events) = glfw
            .create_window(
                u32::from(w),
                u32::from(h),
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Couldn't initialize the window.");
        window.set_resizable(false);
        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        Self {
            glfw,
            window,
            resolution: Resolution::new(w, h),
            visual,
            events,
            frame: vec![0u8; usize::from(w) * usize::from(h) * usize::from(visual)],
        }
    }

    /// Get the internal buffer
    pub fn get_frame(&mut self) -> &mut [u8] {
        &mut self.frame
    }

    fn swap(&mut self) {
        let gl_enum = match self.visual {
            ColorDepth::L8 => GL_LUMINANCE,
            ColorDepth::La8 => GL_LUMINANCE_ALPHA,
            ColorDepth::Rgb8 => GL_RGB,
            ColorDepth::Rgba8 => GL_RGBA,
        };
        unsafe {
            glRasterPos2i(-1, 1);
            glPixelZoom(1., -1.);
            let (width, height) = self.resolution.into();
            #[allow(clippy::as_conversions)]
            glDrawPixels(
                i32::from(width),
                i32::from(height),
                gl_enum,
                GL_UNSIGNED_BYTE,
                self.frame.as_ptr().cast(),
            );
        }
        self.window.swap_buffers();
    }

    /// Show the window
    pub fn show(&mut self) {
        while !self.window.should_close() {
            self.swap();
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    self.window.set_should_close(true);
                }
            }
        }
    }

    /// Add logic while the window is shown
    pub fn shown(&mut self) -> bool {
        self.swap();
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                self.window.set_should_close(true);
            }
        }
        !self.window.should_close()
    }
}

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

#[derive(Clone, Copy)]
pub struct Resolution((u16, u16));
impl Resolution {
    pub const fn new(width: u16, height: u16) -> Self {
        Self((width, height))
    }
}

impl From<(u16, u16)> for Resolution {
    fn from(value: (u16, u16)) -> Self {
        Self(value)
    }
}

impl From<Resolution> for (u16, u16) {
    fn from(value: Resolution) -> Self {
        value.0
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<(i32, i32)> for Resolution {
    fn from(value: (i32, i32)) -> Self {
        let (width, height) = value;
        debug_assert!(width < i32::from(u16::MAX));
        debug_assert!(height < i32::from(u16::MAX));
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        unsafe {
            // SAFETY: assertion in debug builds that the width and height fits into u16.
            Self((
                width.try_into().unwrap_unchecked(),
                height.try_into().unwrap_unchecked(),
            ))
        }
    }
}

#[derive(Clone, Copy)]
pub struct FramebufferCoordinates((u16, u16));

impl From<(u16, u16)> for FramebufferCoordinates {
    fn from(coords: (u16, u16)) -> Self {
        Self(coords)
    }
}
