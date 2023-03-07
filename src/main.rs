use std::cell::RefCell;
use std::rc::Rc;
use crate::application::{AppConfig, Application};
use crate::rope::Mass;
use crate::vector::Vector2d;

mod rope;
mod vector;
mod application;

#[macro_use]
extern crate glium;

fn main() {
    let config = AppConfig::new();
    let app = Application::new(config);

    use glium::{glutin, Surface};
    use glium::glutin::event_loop::ControlFlow;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            gl_PointSize = 5;
        }
    "#;

    let fragment_shader_src_euler = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 1.0, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src_verlet = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.1, 1.0, 1.0, 1.0);
        }
    "#;

    let micros = 1000 * 1000 / app.config.steps_per_frame;
    let line_indices = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);
    let program1 = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src_euler, None).unwrap();
    let program2 = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src_verlet, None).unwrap();

    event_loop.run(move |ev, _, control_flow: &mut ControlFlow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_micros(micros as u64);
        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::Init | glutin::event::StartCause::ResumeTimeReached { .. } => (),
                _ => return,
            },
            _ => return,
        }

        app.update();
        let (w, h) = (app.screen_width / 2, app.screen_height / 2);
        let ms = &app.rope_euler.as_ref().unwrap().masses;
        let cvt = |m: &Rc<RefCell<Mass>>| {
            let pos: Vector2d = m.as_ref().borrow().position;
            Vertex { position: [pos.x as f32 / w as f32, pos.y as f32 / h as f32] }
        };
        let spring1: Vec<Vertex> = ms.iter().map(cvt).collect();
        let spring2: Vec<Vertex> = app.rope_verlet.as_ref().unwrap().masses.iter().map(cvt).collect();

        let mut target = display.draw();
        target.clear_color(0.2, 0.2, 0.2, 0.7);
        let vertex_buffer = glium::VertexBuffer::new(&display, &spring1).unwrap();
        target.draw(&vertex_buffer, &line_indices, &program1, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(&display, &spring2).unwrap();
        target.draw(&vertex_buffer, &line_indices, &program2, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();


        target.finish().unwrap();
    });
}