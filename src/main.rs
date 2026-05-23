use glfw::{self, Context, fail_on_errors};

fn main() {
    println!("Hello, world!");

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Beginner WGPU", glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        window.swap_buffers();
    }
}
