
use egui::Context;
use egui_glium::EguiGlium;
use glium::backend::glutin::SimpleWindowBuilder;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder, ControlFlow},
    window::Window, dpi::PhysicalSize,
};

use crate::prelude::*;

pub trait ApplicationContext {
    const WINDOW_TITLE:&'static str;

    fn new(display: &Display) -> Self;
    fn update(&mut self) { }
    fn handle_window_event(&mut self, event: &WindowEvent, window: &Window);

    fn draw_frame(&mut self, display: &Display, ctx: &mut EguiGlium);
    fn draw_ui(&mut self, ctx: &Context, control_flow: &mut ControlFlow);
}

#[derive(Debug)]
pub struct State<T> {
    pub display: Display,
    pub window: Window,
    pub context: T,

    event_loop: EventLoop<()>
}

impl<T: ApplicationContext + 'static> State<T> {
    pub fn new() -> Self {
        let event_loop = EventLoopBuilder::new().build();
        let (window, display) = SimpleWindowBuilder::new()
            .with_title(T::WINDOW_TITLE)
            .build(&event_loop);

        let context = T::new(&display);

        Self { display, window, context, event_loop }
    }

    /// Start the event_loop and keep rendering frames until the program is closed
    pub fn run_loop(mut self) -> ! {
        let mut ui_ctx = EguiGlium::new(&self.display, &self.window, &self.event_loop);
        
        self.event_loop.run(move |event, _window_target, control_flow| {
            match event {
                // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
                // This is needed, otherwise camera movements can be laggy.
                Event::RedrawEventsCleared => self.window.request_redraw(),

                // set the window size (will call WindowEvent::Resized in the camera)
                // this is a hack to correctly set the inital aspect ratio for the camera
                Event::Resumed => {
                    // TODO: cache window size so that last used window size persists
                    let _ = self.window.set_inner_size(PhysicalSize { width: 800, height: 600 });
                },
                Event::RedrawRequested(_) => {
                    self.context.update();

                    ui_ctx.run(&self.window, |ctx| {
                        self.context.draw_ui(ctx, control_flow);
                    });
                    self.context.draw_frame(&self.display, &mut ui_ctx);
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => self.display.resize(new_size.into()),

                    // Exit the event loop when requested
                    WindowEvent::CloseRequested => control_flow.set_exit(),

                    // dispatch unmatched events to handler
                    event => {
                        if !ui_ctx.on_event(&event).consumed {
                            self.context.handle_window_event(&event, &self.window)
                        }
                    }
                },
                _ => (),
            };
        })
    }
}