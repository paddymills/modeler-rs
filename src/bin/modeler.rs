use std::sync::mpsc;
use std::thread;

use phobia::controls::mouse::MouseController;
use phobia::renderer::Renderer;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

// struct App {
//     renderer: Renderer
// }

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
    
    let (tx, rx) = mpsc::channel();

    let mut renderer = Renderer::new().expect("Cannot create renderer");
    thread::spawn(move || {
        while let Ok(v) = rx.recv() {
            match v {
                (x, y) if x < 0. && y < 0. => renderer.draw(),
                (x, y) => renderer.add_point(x, y)
            }
        }
    });
    
    let mut mouse = MouseController::new(tx.clone());

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
                
                WindowEvent::CursorMoved { .. } |
                WindowEvent::MouseInput  { .. } => {
                    mouse.handle_event(event, gl_context.window())
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let _ = tx.send((-1., -1.));
                gl_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
