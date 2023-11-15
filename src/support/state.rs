
use glium::{
    Display,
    glutin::surface::WindowSurface,
    backend::glutin::SimpleWindowBuilder,
};

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    keyboard::{Key, NamedKey},
    window::Window, dpi::PhysicalSize,
};

pub trait ApplicationContext {
    const WINDOW_TITLE:&'static str;
    fn draw_frame(&mut self, _display: &Display<WindowSurface>) { }
    fn new(display: &Display<WindowSurface>) -> Self;
    fn update(&mut self) { }
    fn handle_window_event(&mut self, _event: &WindowEvent, _window: &winit::window::Window) { }
}

pub struct State<T> {
    pub display: Display<WindowSurface>,
    pub window: Window,
    pub context: T,
    active: bool
}

impl<T: ApplicationContext + 'static> State<T> {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title(crate::config::TITLE)
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

        event_loop.run(move |event, window_target| {
            if !state.active { () }

            match event {
                Event::Suspended => state.active = false,
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                Event::AboutToWait => {
                    state.window.request_redraw();
                },
                Event::Resumed => {
                    // set the window size (will call WindowEvent::Resized in the camera)
                    // this is a hack to correctly set the inital aspect ratio for the camera
                    let _ = state.window.request_inner_size(PhysicalSize { width: 800, height: 600 });
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => {
                        state.display.resize(new_size.into());
                    },
                    WindowEvent::RedrawRequested => {
                        state.context.update();
                        state.context.draw_frame(&state.display);
                    },
                    // Exit the event loop when requested (by closing the window for example) or when
                    // pressing the Esc key.
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput { event: KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    }, ..} => {
                        window_target.exit()
                    },
                    // Every other event
                    ev => {
                        state.context.handle_window_event(&ev, &state.window);
                    },
                },
                _ => (),
            };
        })
    }
}