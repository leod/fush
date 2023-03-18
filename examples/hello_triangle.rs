use std::time::Instant;

use posh::{gl, sl, Block, BlockDom, Sl};

// Shader interface

#[derive(Clone, Copy, Block)]
struct Globals<D: BlockDom = Sl> {
    time: D::F32,
}

// Shader code

fn vertex_shader(_: (), vertex: sl::Vec2) -> sl::VaryingOutput<sl::Vec2> {
    let vertex = vertex - sl::vec2(0.5, 0.5);

    sl::VaryingOutput {
        varying: vertex,
        position: sl::vec4(vertex.x, vertex.y, 0.0, 1.0),
    }
}

fn fragment_shader(uniform: Globals, varying: sl::Vec2) -> sl::Vec4 {
    let rg = (varying + uniform.time).cos().pow(sl::vec2(2.0, 2.0));

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

// Host code

struct Demo {
    program: gl::Program<Globals, sl::Vec2>,

    globals: gl::UniformBuffer<Globals>,
    vertices: gl::VertexBuffer<sl::Vec2>,

    start_time: Instant,
}

impl Demo {
    pub fn new(context: gl::Context) -> Result<Self, gl::CreateError> {
        let program = context.create_program(vertex_shader, fragment_shader)?;

        let globals =
            context.create_uniform_buffer(Globals { time: 0.0 }, gl::BufferUsage::StreamDraw)?;
        let vertices = context.create_vertex_buffer(
            &[[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()],
            gl::BufferUsage::StaticDraw,
        )?;

        let start_time = Instant::now();

        Ok(Self {
            program,
            globals,
            vertices,
            start_time,
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        self.globals.set(Globals {
            time: Instant::now().duration_since(self.start_time).as_secs_f32(),
        });

        self.program.draw(
            self.globals.as_binding(),
            gl::VertexStream {
                vertices: self.vertices.as_binding(),
                elements: gl::Elements::Range(0..3),
                primitive: gl::PrimitiveType::Triangles,
            },
            gl::DefaultFramebuffer::default(),
            gl::DrawParams::default().with_clear_color(glam::vec4(0.1, 0.2, 0.3, 1.0)),
        )
    }
}

// SDL glue

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Hello triangle!", 1024, 768)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let context = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let context = gl::Context::new(context).unwrap();
    let demo = Demo::new(context).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            if matches!(event, Quit { .. }) {
                running = false;
            }
        }

        demo.draw().unwrap();
        window.gl_swap_window();
    }
}
