use phobia::renderer::Renderer;
use glutin::event::{Event, WindowEvent, ElementState, MouseButton};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title(phobia::config::TITLE);

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    
    // TODO: mouse handler to struct
    let (mut x, mut y) = (0f32, 0f32);
    
    let mut renderer = Renderer::new().expect("Cannot create renderer");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    gl_context.resize(physical_size);
                    gl_context.window().request_redraw();
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::CursorMoved { position, .. } => {
                    x = position.x as f32;
                    y = position.y as f32;
                },
                WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                    let win = gl_context.window().inner_size();
                    let x = -1. + 2. * x / win.width  as f32;
                    let y =  1. - 2. * y / win.height as f32;

                    renderer.add_point(x, y);
                    gl_context.window().request_redraw();
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.draw();
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
