
use glium::backend::glutin::SimpleWindowBuilder;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoopBuilder, ControlFlow},
    window::Window, dpi::{PhysicalSize, PhysicalPosition},
};

use crate::prelude::*;
pub type EventLoop = winit::event_loop::EventLoop<()>;

pub trait ApplicationState {
    fn new(display: &Display, window: &Window, event_loop: &EventLoop) -> Self;
    fn update(&mut self) { }
    fn handle_window_event(&mut self, event: &WindowEvent, window: &Window);

    // TODO: combine these
    fn draw_frame(&mut self, display: &Display);
    fn draw_ui(&mut self, control_flow: &mut ControlFlow, window: &Window);
}

#[derive(Debug)]
pub struct Application<S> {
    display: Display,
    window: Window,
    event_loop: EventLoop,
    state: S
}

impl<S: ApplicationState + 'static> Application<S> {
    /// create application and run it
    pub fn run() -> ! {
        crate::prelude::register();
        let app = Self::new();

        app.event_loop()
    }

    fn new() -> Self {
        let event_loop = EventLoopBuilder::new().build();
        let (window, display) = SimpleWindowBuilder::new()
            .with_title(crate::config::TITLE)
            .build(&event_loop);

        let state = S::new(&display, &window, &event_loop);

        Self { display, window, event_loop, state }
    }

    fn event_loop(mut self) -> ! {
        self.event_loop.run(move |event, _window_target, control_flow| {
            match event {
                // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
                // This is needed, otherwise camera movements can be laggy.
                Event::RedrawEventsCleared => self.window.request_redraw(),

                // set the window size (will call WindowEvent::Resized in the camera)
                // this is a hack to correctly set the inital aspect ratio for the camera
                Event::Resumed => {
                    let mon = self.window.current_monitor().unwrap().size();
                    // TODO: cache window size so that last used window size persists
                    let size = PhysicalSize { width: 800u32, height: 600u32 };
                    let pos = PhysicalPosition { x: (mon.width - size.width) / 2, y: (mon.height - size.height) / 2 };

                    let _ = self.window.set_inner_size(size);
                    let _ = self.window.set_outer_position(pos);
                },
                Event::RedrawRequested(_) => {
                    self.state.update();

                    self.state.draw_ui(control_flow, &self.window);
                    self.state.draw_frame(&self.display);
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => self.display.resize(new_size.into()),

                    // Exit the event loop when requested
                    WindowEvent::CloseRequested => control_flow.set_exit(),

                    // dispatch unmatched events to handler
                    event => self.state.handle_window_event(&event, &self.window)
                },
                _ => (),
            }
        })
    }
}
