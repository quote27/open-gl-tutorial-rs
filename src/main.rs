extern crate gl;
extern crate glfw;
extern crate cgmath;
extern crate time;

use time::precise_time_s;
use gl::types::*;
use glfw::{Action, Context, Key};
use std::mem;
use std::ptr;
use shaders::{Shader, Program, Uniform};

mod shaders;

static VS_SRC: &'static str = "
#version 150 core
in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}";

static FS_SRC: &'static str = "
#version 150 core
uniform vec3 triangle_color;
out vec4 out_color;

void main() {
    out_color = vec4(triangle_color, 1.0);
}";

fn main() {
    println!("open.gl tutorial begin");
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw.create_window(300, 300, "open.gl tutorial", glfw::WindowMode::Windowed)
        .expect("failed to create glfw window");

    gl::load_with(|s| window.get_proc_address(s));

    window.set_key_polling(true);
    window.make_current();


    println!("creating shaders");
    let shaders_v = vec![
        Shader::from_str(gl::VERTEX_SHADER, &VS_SRC),
        Shader::from_str(gl::FRAGMENT_SHADER, &FS_SRC),
    ];
    gl_error();

    println!("creating program");
    let prog = Program::new(&shaders_v);
    gl_error();

    println!("creating vertex array object (vao)");
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }
    gl_error();


    let vertices: [f32; 6] = [
        0.0,  0.5, // vertex 1
        0.5, -0.5, // vertex 2
       -0.5, -0.5, // vertex 3
    ];

    // upload data to card
    println!("creating vertex buffer object (vbo)");
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&vertices[0]), gl::STATIC_DRAW);
    }
    gl_error();


    prog.use_prog();
    let pos_attr = prog.get_attrib("position");
    println!("position attribute: {}", pos_attr);
    gl_error();

    println!("setting vertex attribut pointer and enabling enabling vertex attrib array");
    unsafe {
        let pos_attr_u = pos_attr as GLuint;
        println!("  enable vertex attrib array");
        gl::EnableVertexAttribArray(pos_attr_u);
        gl_error();
        println!("  vertex attrib pointer");
        gl::VertexAttribPointer(pos_attr_u, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl_error();
    }

    let triangle_color_u = prog.get_unif("triangle_color");

    let t_start = precise_time_s();

    println!("starting main loop");
    while !window.should_close() {
        let t_now = precise_time_s();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // update scene
        let t_diff = t_now - t_start;
        triangle_color_u.upload_3f(((t_diff * 4.0).sin() as f32 + 1.0) / 2.0, 0.0, 0.0);

        // draw graphics
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        // present graphics

        window.swap_buffers();
    }

    println!("open.gl tutorial end");
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}

fn gl_error() {
    let er = unsafe { gl::GetError() };
    if er != 0 {
        println!("gl error? {}", er);
    }
}
