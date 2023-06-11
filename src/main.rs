#[macro_use] //allows us to use macros from glium
#[allow(unused_imports)] //allows us to import glutin without getting a warning
extern crate glium;
use glium::glutin;
use glium::Surface;
use glm::Vec2;
use nalgebra_glm as glm;
mod shader;

#[derive(Copy, Clone)] //automatically create the implementation of the trait `Copy` and `Clone` for the struct `Vertex`
pub struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
pub fn create_vertex(vec2: Vec2) -> Vertex {
    return Vertex {
        position: [vec2.x, vec2.y],
    };
}

fn create_window(start_time: std::time::Instant) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // let vertex_buffer: glium::VertexBuffer<Vertex> =
    //     glium::VertexBuffer::empty_dynamic(&display, 10).unwrap();
    let shape = vec![
        create_vertex(Vec2::new(-1.0, -1.0)),
        create_vertex(Vec2::new(1.0, -1.0)),
        create_vertex(Vec2::new(-1.0, 1.0)),
        create_vertex(Vec2::new(1.0, -1.0)),
        create_vertex(Vec2::new(1.0, 1.0)),
        create_vertex(Vec2::new(-1.0, 1.0)),
        // (-1.0,1.0)         (1.0,1.0)
        //     +---------------+
        //     +---------------+
        // (-1.0,-1.0)         (1.0,-1.0)
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = shader::get_vertex_shader();
    let fragment_shader_src = shader::get_fragment_shader();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        let program =
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let time_elapsed = start_time.elapsed().as_secs_f32();
        let (vwidth, vheight) = display.get_framebuffer_dimensions();

        let uniforms = uniform! {
            time: time_elapsed,
            viewport_width: vwidth,
            viewport_height: vheight,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

fn main() {
    let start_of_program = std::time::Instant::now();
    create_window(start_of_program);
}
