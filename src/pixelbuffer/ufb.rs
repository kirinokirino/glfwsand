extern crate glfw;
use glu_sys::glu::{glDrawPixels, glPixelZoom, glRasterPos2i, GL_RGB, GL_UNSIGNED_BYTE};

use glfw::{Action, Context, Key, MouseButton};
use std::convert::{From, Into};

use crate::gfx::Pixel;

/// Wrapper around a glfw window
pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    resolution: Resolution,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    frame: Option<Vec<Pixel>>,
}

impl Window {
    /// Instantiate a window
    #[must_use]
    pub fn new(res: Resolution, title: &str) -> Self {
        let (w, h) = res.into();
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
        // Polling https://github.com/PistonDevelopers/glfw-rs/blob/master/examples/events.rs
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_close_polling(true);
        window.set_mouse_button_polling(true);
        window.make_current();
        glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        Self {
            glfw,
            window,
            resolution: Resolution::new(w, h),
            events,
            frame: Some(vec![Pixel::new(0, 50, 80); usize::from(w) * usize::from(h)]),
        }
    }

    /// Get the internal buffer
    pub fn set_frame(&mut self, frame: Vec<Pixel>) {
        self.frame = Some(frame);
    }

    fn swap(&mut self) {
        if let Some(ref frame) = self.frame {
            unsafe {
                glRasterPos2i(-1, 1);
                glPixelZoom(1., -1.);
                let (width, height) = self.resolution.into();
                glDrawPixels(
                    i32::from(width),
                    i32::from(height),
                    GL_RGB,
                    GL_UNSIGNED_BYTE,
                    frame.as_ptr().cast(),
                );
            }
            self.window.swap_buffers();
        }
    }

    /// Add logic while the window is shown
    pub fn shown(&mut self) -> Vec<Event> {
        self.swap();
        self.glfw.poll_events();
        let mut events = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)
                | glfw::WindowEvent::Close => {
                    self.window.set_should_close(true);
                    events.push(Event::Close);
                }
                glfw::WindowEvent::Key(key, _, Action::Press, _) => events.push(Event::Key(key)),
                glfw::WindowEvent::CursorPos(x, y) => events.push(Event::Cursor((x, y))),
                glfw::WindowEvent::MouseButton(btn, _action, _mods) => {
                    events.push(Event::MouseButton(btn));
                }
                _ => (),
            };
        }
        events
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Key(Key),
    MouseButton(MouseButton),
    Cursor((f64, f64)),
    Close,
}

#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

impl Resolution {
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    #[must_use]
    pub fn area(self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }

    #[must_use]
    pub const fn center(self) -> (u16, u16) {
        #[allow(clippy::integer_division)]
        (self.width / 2, self.height / 2)
    }
}

impl From<(u16, u16)> for Resolution {
    fn from(value: (u16, u16)) -> Self {
        Self {
            width: value.0,
            height: value.1,
        }
    }
}

impl From<Resolution> for (u16, u16) {
    fn from(resolution: Resolution) -> Self {
        (resolution.width, resolution.height)
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
            Self {
                width: width.try_into().unwrap_unchecked(),
                height: height.try_into().unwrap_unchecked(),
            }
        }
    }
}
