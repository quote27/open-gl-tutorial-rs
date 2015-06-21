extern crate gl;
extern crate glfw;
extern crate cgmath;
extern crate time;
extern crate image;

use time::precise_time_s;
use gl::types::*;
use glfw::{Action, Context, Key};
use image::GenericImage;
use cgmath::*;
use std::mem;
use std::ptr;
use shaders::{Shader, Program, Uniform};

mod shaders;

static VS_SRC: &'static str = "
#version 150 core
in vec3 position;
in vec3 color;
in vec2 texcoord;

out vec3 o_color;
out vec2 o_texcoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

void main() {
    o_color = color;
    o_texcoord = texcoord;
    gl_Position = proj * view * model * vec4(position, 1.0);
}";

static FS_SRC: &'static str = "
#version 150 core
in vec3 o_color;
in vec2 o_texcoord;
out vec4 out_color;

uniform sampler2D tex_kitty;
uniform sampler2D tex_puppy;

uniform float alpha;

void main() {
    vec4 col_a = texture(tex_kitty, o_texcoord);
    vec4 col_b = texture(tex_puppy, o_texcoord);
    out_color = mix(col_a, col_b, 0.5) * vec4(o_color, 1.0); // * alpha;
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


    let vertices = [
       -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
        0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
        0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
       -0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
       -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 0.0,

       -0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
        0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
        0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
       -0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
       -0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 0.0,

       -0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
       -0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
       -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
       -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
       -0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
       -0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,

        0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
        0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
        0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
        0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
        0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,

       -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
        0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
        0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
       -0.5, -0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
       -0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,

       -0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0,
        0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
        0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 1.0, 0.0,
       -0.5,  0.5,  0.5, 1.0, 1.0, 1.0, 0.0, 0.0,
       -0.5,  0.5, -0.5, 1.0, 1.0, 1.0, 0.0, 1.0f32,
    ];
    let vertex_size = 8;

    let elements = [
        0, 1, 2,
        2, 3, 0u32,
    ];

    // upload data to card
    println!("vertices: creating vertex buffer object (vbo)");
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr, mem::transmute(&vertices[0]), gl::STATIC_DRAW);
    }
    gl_error();

    // upload data to card
    println!("elements: creating vertex buffer object (ebo)");
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (elements.len() * mem::size_of::<u32>()) as GLsizeiptr, mem::transmute(&elements[0]), gl::STATIC_DRAW);
    }
    gl_error();


    prog.use_prog();

    let pos_attr = prog.get_attrib("position");
    println!("position: attribute: {}", pos_attr);
    gl_error();

    println!("position: setting vertex attribute pointer and enabling enabling vertex attrib array");
    unsafe {
        let pos_attr_u = pos_attr as GLuint;
        println!("  enable vertex attrib array");
        gl::EnableVertexAttribArray(pos_attr_u);
        gl_error();
        println!("  vertex attrib pointer");
        gl::VertexAttribPointer(pos_attr_u, 3, gl::FLOAT, gl::FALSE, (vertex_size * mem::size_of::<f32>()) as GLint, ptr::null());
        gl_error();
    }

    let color_attr = prog.get_attrib("color");
    println!("color: attribute: {}", pos_attr);
    gl_error();

    println!("color: setting vertex attribute pointer and enabling enabling vertex attrib array");
    unsafe {
        let color_attr_u = color_attr as GLuint;
        println!("  enable vertex attrib array");
        gl::EnableVertexAttribArray(color_attr_u);
        gl_error();
        println!("  vertex attrib pointer");
        gl::VertexAttribPointer(color_attr_u, 3, gl::FLOAT, gl::FALSE, (vertex_size * mem::size_of::<f32>()) as GLint, mem::transmute(3 * mem::size_of::<f32>()));
        gl_error();
    }

    let texcoord_attr = prog.get_attrib("texcoord");
    println!("texcoord: attribute: {}", pos_attr);
    gl_error();

    println!("texcoord: setting vertex attribute pointer and enabling enabling vertex attrib array");
    unsafe {
        let texcoord_attr_u = texcoord_attr as GLuint;
        println!("  enable vertex attrib array");
        gl::EnableVertexAttribArray(texcoord_attr_u);
        gl_error();
        println!("  vertex attrib pointer");
        gl::VertexAttribPointer(texcoord_attr_u, 2, gl::FLOAT, gl::FALSE, (vertex_size * mem::size_of::<f32>()) as GLint, mem::transmute(6 * mem::size_of::<f32>()));
        gl_error();
    }


    println!("loading textures");
    let mut textures: [GLuint; 2] = [0, 0];
    unsafe {
        use std::path::Path;

        let img0 = image::open(&Path::new("data/sample.png")).unwrap();
        let (w0, h0) = img0.dimensions();
        let img1 = image::open(&Path::new("data/sample2.png")).unwrap();
        let (w1, h1) = img1.dimensions();

        gl::GenTextures(2, mem::transmute(&textures[0]));

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, w0 as i32, h0 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(img0.raw_pixels().as_ptr()));
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        prog.get_unif("tex_kitty").upload_1i(0);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, w1 as i32, h1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(img1.raw_pixels().as_ptr()));
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        prog.get_unif("tex_puppy").upload_1i(1);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let alpha_u = prog.get_unif("alpha");
    let model_u = prog.get_unif("model");
    let view_u = prog.get_unif("view");
    let proj_u = prog.get_unif("proj");

    let mut proj_m4 = perspective(deg(45.0), 800.0 / 600.0, 1.0, 10.0);
    let mut view_m4 = Matrix4::look_at(&Point3::new(1.2, 1.2, 1.2), &Point3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 0.0, 1.0));

    proj_u.upload_m4f(&proj_m4);
    view_u.upload_m4f(&view_m4);


    let t_start = precise_time_s();

    println!("starting main loop");
    while !window.should_close() {
        let t_now = precise_time_s();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // clear
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // update scene
        let t_diff = t_now - t_start;
        alpha_u.upload_1f(((t_diff * 4.0).sin() as f32 + 1.0) / 2.0);

        let rot180 = Basis3::from_axis_angle(&Vector3::new(0.0, 0.0, 1.0), deg(180.0 * t_diff as f32).into());
        let mut model_mat4 = Matrix4::identity();
        model_mat4 = model_mat4 * Matrix4::from(*rot180.as_ref());
        model_u.upload_m4f(&model_mat4);

        // draw graphics
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

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
