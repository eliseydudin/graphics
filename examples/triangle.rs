use graphics::{attributes, ClearFlags, Color, DrawMode, Program, Shader, Vao, Vbo};
use sdl2::event::Event;

const VERTEX_SOURCE: &'static str = r#"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 fragment_color;
void main() {
    gl_Position = vec4(position, 1.0, 1.0);
    fragment_color = color;
}
"#;

const FRAGMENT_SOURCE: &'static str = r#"
#version 330 core
in vec3 fragment_color;
out vec4 color;
void main() {
    color = vec4(fragment_color, 1.0);
}
"#;

const TRIANGLE_DATA: [f32; 15] = [
    0.0, 0.5, // first pos
    1.0, 0.819, 0.729, // first color
    0.5, -0.5, // second pos
    0.807, 0.490, 0.647, // second color
    -0.5, -0.5, // third pos
    0.745, 0.8980, 0.749, // third color
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window = video
        .window("Graphics", 960, 720)
        .allow_highdpi()
        .opengl()
        .resizable()
        .build()?;

    let attr = video.gl_attr();
    attr.set_context_major_version(3);
    attr.set_context_minor_version(3);
    attr.set_context_profile(sdl2::video::GLProfile::Core);
    let gl_context = window.gl_create_context()?;

    window.gl_make_current(&gl_context)?;

    let draw_layer = unsafe { graphics::DrawLayer::new(|s| video.gl_get_proc_address(s)) };
    draw_layer.set_clear_color(Color::WHITE);

    let vao = Vao::new();
    let vbo = Vbo::new(&vao);
    vbo.bind_data(&TRIANGLE_DATA);

    let vertex = Shader::compile(&VERTEX_SOURCE)?;
    let fragment = Shader::compile(&FRAGMENT_SOURCE)?;
    let program = Program::new(vertex, fragment)?;

    let attrs = attributes! {
        position: vec<f32, 2>,
        color: vec<f32, 3>
    };
    attrs
        .calculate_for(&program)
        .ok_or("Failed to describe the memory layout")?;

    draw_layer.use_program(&program);

    let mut event_pump = sdl.event_pump()?;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => (),
            }
        }

        draw_layer.clear(ClearFlags::COLOR);
        draw_layer.draw_arrays(&vao, DrawMode::Triangles, 0, 3);

        window.gl_swap_window();
    }

    Ok(())
}
