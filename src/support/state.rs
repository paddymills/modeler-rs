
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
    fn draw_frame(&mut self, display: &Display<WindowSurface>, ctx: &mut egui_glium::EguiGlium);
    fn new(display: &Display<WindowSurface>) -> Self;
    fn init(&mut self);
    fn update(&mut self) { }
    fn handle_window_event(&mut self, event: &WindowEvent, window: &winit::window::Window);
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
    pub fn run_loop() -> Result<(), String> {
        let event_loop = EventLoopBuilder::new().build();
        let mut state: State<T> = State::new(&event_loop);
        state.context.init();

        let mut egui_glium_ctx = egui_glium::EguiGlium::new(&state.display, &state.window, &event_loop);

        event_loop.run(move |event, window_target, control_flow| {
            if !state.active { () }

            let mut redraw = || {
                let repaint_after = egui_glium_ctx.run(&state.window, |ctx| {
                    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
                        if ui.button("Menu").clicked() {
                            println!("menu selected");
                        }
                    });
                });

                if repaint_after.is_zero() {
                    let _ = &state.window.request_redraw();
                }
            };

            match event {
                Event::Suspended => state.active = false,
                
                // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
                // This is needed, otherwise camera movements can be laggy.
                Event::RedrawEventsCleared => state.window.request_redraw(),

                // set the window size (will call WindowEvent::Resized in the camera)
                // this is a hack to correctly set the inital aspect ratio for the camera
                Event::Resumed => {
                    // TODO: cache window size so that last used window size persists
                    let _ = state.window.set_inner_size(PhysicalSize { width: 800, height: 600 });
                },
                Event::RedrawRequested(_) => {
                    redraw();

                    state.context.update();
                    state.context.draw_frame(&state.display, &mut egui_glium_ctx);
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => state.display.resize(new_size.into()),

                    // Exit the event loop when requested
                    WindowEvent::CloseRequested => control_flow.set_exit(),

                    // dispatch unmatched events to handler
                    event => {
                        if egui_glium_ctx.on_event(&event).repaint {
                            let _ = &state.window.request_redraw();
                        }
                        state.context.handle_window_event(&event, &state.window)
                    }
                },
                _ => (),
            };
        })
    }
}