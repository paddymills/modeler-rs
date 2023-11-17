
use glium::{
    Display,
    glutin::surface::WindowSurface,
    backend::glutin::SimpleWindowBuilder,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    window::Window, dpi::PhysicalSize,
};

pub trait ApplicationContext {
    const WINDOW_TITLE:&'static str;
    fn draw_frame(&mut self, _display: &Display<WindowSurface>) { }
    fn new(display: &Display<WindowSurface>) -> Self;
    fn init(&mut self);
    fn update(&mut self) { }
    fn handle_window_event(&mut self, _event: &WindowEvent, _window: &winit::window::Window) { }
}

#[derive(Debug)]
pub struct State<T> {
    pub display: Display<WindowSurface>,
    pub window: Window,
    pub context: T,
    active: bool
}

impl<T: ApplicationContext + 'static> State<T> {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title(T::WINDOW_TITLE)
            .build(event_loop);

        Self::from_display_window(display, window)
    }

    pub fn from_display_window(display: Display<WindowSurface>, window: Window) -> Self {
        let context = T::new(&display);

        Self { display, window, context, active: true }
    }

    /// Start the event_loop and keep rendering frames until the program is closed
    pub fn run_loop() -> Result<(), winit::error::EventLoopError> {
        let event_loop = EventLoopBuilder::new()
            .build()
            .expect("event loop building");
        let mut state: State<T> = State::new(&event_loop);
        state.context.init();

        event_loop.run(move |event, window_target| {
            if !state.active { () }

            match event {
                Event::Suspended => state.active = false,
                
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // This is needed, otherwise camera movements can be laggy.
                Event::AboutToWait => state.window.request_redraw(),

                // set the window size (will call WindowEvent::Resized in the camera)
                // this is a hack to correctly set the inital aspect ratio for the camera
                Event::Resumed => {
                    // TODO: cache window size so that last used window size persists
                    let _ = state.window.request_inner_size(PhysicalSize { width: 800, height: 600 });
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => state.display.resize(new_size.into()),
                    WindowEvent::RedrawRequested => {
                        state.context.update();
                        state.context.draw_frame(&state.display);
                    },

                    // Exit the event loop when requested
                    WindowEvent::CloseRequested => window_target.exit(),

                    // dispatch unmatched events to handler
                    event => state.context.handle_window_event(&event, &state.window)
                },
                _ => (),
            };
        })
    }
}