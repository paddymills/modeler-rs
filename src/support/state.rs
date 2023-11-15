
use std::num::NonZeroU32;

use glium::{
    implement_vertex,
    Display,
    glutin::surface::WindowSurface,
    backend::glutin::SimpleWindowBuilder,
};

use glutin::{
    context::NotCurrentGlContext,
    display::{GlDisplay, GetGlDisplay},
};

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    keyboard::{Key, NamedKey},
    raw_window_handle::HasRawWindowHandle,
    window::Window,
};

pub trait ApplicationContext {
    const WINDOW_TITLE:&'static str;
    fn draw_frame(&mut self, _display: &Display<WindowSurface>) { }
    fn new(display: &Display<WindowSurface>) -> Self;
    fn update(&mut self) { }
    fn handle_window_event(&mut self, _event: &WindowEvent, _window: &winit::window::Window) { }
}

pub struct State<T> {
    pub display: glium::Display<WindowSurface>,
    pub window: winit::window::Window,
    pub context: T,
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

        Self { display, window, context }
    }

    /// Start the event_loop and keep rendering frames until the program is closed
    pub fn run_loop() {
        let event_loop = EventLoopBuilder::new()
            .build()
            .expect("event loop building");
        let mut state: Option<State<T>> = Some(State::new(&event_loop));

        let result = event_loop.run(move |event, window_target| {
            match event {
                Event::Suspended => state = None,
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                Event::AboutToWait => {
                    if let Some(state) = &state {
                        state.window.request_redraw();
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => {
                        if let Some(state) = &state {
                            state.display.resize(new_size.into());
                        }
                    },
                    WindowEvent::RedrawRequested => {
                        if let Some(state) = &mut state {
                            state.context.update();
                            state.context.draw_frame(&state.display);
                        }
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
                        if let Some(state) = &mut state {
                            state.context.handle_window_event(&ev, &state.window);
                        }
                    },
                },
                _ => (),
            };
        });
        result.unwrap();
    }
}