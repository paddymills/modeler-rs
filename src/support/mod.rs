
use glium::implement_vertex;
use glium::{self, Display};
use glium::glutin::surface::WindowSurface;

pub mod camera;

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(display: &Display<WindowSurface>, data: &[u8]) -> glium::vertex::VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                },
            }
        }
    }

    glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into()
}

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub trait ApplicationContext {
    fn draw_frame(&mut self, _display: &Display<WindowSurface>) { }
    fn new(display: &Display<WindowSurface>) -> Self;
    fn update(&mut self) { }
    fn handle_window_event(&mut self, _event: &winit::event::WindowEvent, _window: &winit::window::Window) { }
    const WINDOW_TITLE:&'static str;
}

pub struct State<T> {
    pub display: glium::Display<WindowSurface>,
    pub window: winit::window::Window,
    pub context: T,
}

impl<T: ApplicationContext + 'static> State<T> {

    /// Start the event_loop and keep rendering frames until the program is closed
    pub fn run_loop() {
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title(crate::config::TITLE)
            .build(&event_loop);
        let mut state: Option<State<T>> = None;

        let result = event_loop.run(move |event, window_target| {
            match event {
                winit::event::Event::Suspended => state = None,
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                winit::event::Event::AboutToWait => {
                    if let Some(state) = &state {
                        state.window.request_redraw();
                    }
                }
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::Resized(new_size) => {
                        if let Some(state) = &state {
                            state.display.resize(new_size.into());
                        }
                    },
                    winit::event::WindowEvent::RedrawRequested => {
                        if let Some(state) = &mut state {
                            state.context.update();
                            state.context.draw_frame(&state.display);
                        }
                    },
                    // Exit the event loop when requested (by closing the window for example) or when
                    // pressing the Esc key.
                    winit::event::WindowEvent::CloseRequested
                    | winit::event::WindowEvent::KeyboardInput { event: winit::event::KeyEvent {
                        state: winit::event::ElementState::Pressed,
                        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
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
