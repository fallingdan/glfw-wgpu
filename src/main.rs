use glfw::{self, Context, Window, fail_on_errors};

struct State<'a> {
    instance: wgpu::Instance,   // Our initial access to wgpu
    surface: wgpu::Surface<'a>, // The "screen" we will be writing to
    device: wgpu::Device,       // The handle for the GPU
    queue: wgpu::Queue,         // Where we will submit work to get done
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window,
}

impl<'a> State {
    async fn new(window: &'a mut Window) -> Self {
        let size = window.get_size();

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            backend_options: wgpu::BackendOptions::default(),
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            display: None,
        };

        let instance = wgpu::Instance::new(instance_descriptor);

        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window) }.unwrap();
        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };

        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();
    }
}

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
