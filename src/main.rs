extern crate gl;
extern crate glfw;
extern crate cgmath;

use gl::types::*;

use glfw::{Action, Context, Key};

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

    {
        let mut vert_buf = 0;
        unsafe { gl::GenBuffers(1, &mut vert_buf); }

        assert!(vert_buf == 1, "gen buffers returned an incorrect value: {}", vert_buf);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // update scene

        // draw graphics

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
