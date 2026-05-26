use glfw::{self, Context, fail_on_errors};

fn main() {
    println!("Hello, world!");

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Beginner WGPU", glfw::WindowMode::Windowed)
        .unwrap();

    // Poll against key events
    window.set_key_polling(true);

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                }
                e => {
                    println!("{:?}", e);
                }
            }
        }

        window.swap_buffers();
    }
}
