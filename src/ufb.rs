extern crate glfw;
use glu_sys::glu::{
    glDrawPixels, glPixelZoom, glRasterPos2i, GL_LUMINANCE, GL_LUMINANCE_ALPHA, GL_RGB, GL_RGBA,
    GL_UNSIGNED_BYTE,
};

use glfw::{Action, Context, Key};
use std::convert::From;

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
    win: glfw::Window,
    w: u16,
    h: u16,
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
            win: window,
            w,
            h,
            visual,
            events,
            frame: vec![0u8; usize::from(w) * usize::from(h) * usize::from(visual)],
        }
    }

    /// Get the internal buffer
    pub fn get_frame(&mut self) -> &mut [u8] {
        &mut self.frame
    }

    /// Get the pixels at `x, y`
    pub fn buffer_index_at(&self, x: u16, y: u16) -> Option<usize> {
        if x >= self.w || y >= self.h {
            None
        } else {
            Some((usize::from(y) * usize::from(self.w) + usize::from(x)) * usize::from(self.visual))
        }
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
            #[allow(clippy::as_conversions)]
            glDrawPixels(
                self.w as _,
                self.h as _,
                gl_enum,
                GL_UNSIGNED_BYTE,
                self.frame.as_ptr() as _,
            );
        }
        self.win.swap_buffers();
    }

    /// Show the window
    pub fn show(&mut self) {
        while !self.win.should_close() {
            self.swap();
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    self.win.set_should_close(true);
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
                self.win.set_should_close(true);
            }
        }
        !self.win.should_close()
    }
}
