use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, UNIX_EPOCH};

use embedded_graphics_core::geometry::{OriginDimensions, Point, Size};
use embedded_graphics_core::pixelcolor::raw::RawU16;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::prelude::DrawTarget;
use embedded_graphics_core::primitives::Rectangle;
use slint::platform::software_renderer::{LineBufferProvider, Rgb565Pixel};

use crate::display::DISPLAY_WIDTH;
pub trait DISPLAY: DrawTarget<Color = Rgb565> + OriginDimensions {}
impl<T: DrawTarget<Color = Rgb565> + OriginDimensions> DISPLAY for T {}
#[derive(Clone)]
pub struct EspBackend<D: DISPLAY> {
    window: RefCell<Option<Rc<slint::platform::software_renderer::MinimalSoftwareWindow>>>,
    display: RefCell<D>,
    size: Size,
}

impl<D: DISPLAY> EspBackend<D> {
    pub fn new(display: D) -> Self {
        let size = display.size();
        Self {
            window: RefCell::new(None),
            display: RefCell::new(display),
            size,
        }
    }
}
struct DrawBuffer<'a, D: DISPLAY> {
    display: &'a mut D,
    buffer: &'a mut [Rgb565Pixel],
}

impl<D: DISPLAY> slint::platform::Platform for EspBackend<D> {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
            slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
        );
        self.window.replace(Some(window.clone()));
        Ok(window)
    }

    fn run_event_loop(&self) -> anyhow::Result<(), slint::PlatformError> {
        let size = slint::PhysicalSize::new(self.size.width, self.size.height);
        let window = self.window.borrow().clone().unwrap();
        window.set_size(size);
        let mut buf = [Rgb565Pixel(0); DISPLAY_WIDTH as usize];
        let mut buffer = DrawBuffer {
            display: &mut *self.display.borrow_mut(),
            buffer: &mut buf,
        };
        loop {
            std::thread::sleep(Duration::from_millis(10));
            slint::platform::update_timers_and_animations();
            window.draw_if_needed(|renderer| {
                renderer.render_by_line(&mut buffer);
            });
            if window.has_active_animations() {
                continue;
            }
        }
    }

    fn duration_since_start(&self) -> Duration {
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
    }

    fn debug_log(&self, arguments: core::fmt::Arguments) {
        println!("{}", arguments);
    }
}

impl<'a, D: DISPLAY> LineBufferProvider for &mut DrawBuffer<'a, D> {
    type TargetPixel = Rgb565Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        let buffer = &mut self.buffer[range.clone()];
        render_fn(buffer);
        self.display
            .fill_contiguous(
                &Rectangle::new(
                    Point::new(range.start as _, line as _),
                    Size::new(range.len() as _, 1),
                ),
                buffer.iter().map(|p| Rgb565::from(RawU16::new(p.0))),
            )
            .map_err(drop)
            .unwrap();
    }
}
