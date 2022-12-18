use beryllium::{Sdl, init::InitFlags, video::{GlProfile, GlContextFlags, CreateWinArgs}, events::Event};
use gl33::{global_loader::{glClearColor, glClear, load_global_gl, glClearBufferfv}};

fn main() {
    let sdl = Sdl::init(InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl.set_gl_context_flags(GlContextFlags::FORWARD_COMPATIBLE).unwrap();
    }

    let _win = sdl.create_gl_window(CreateWinArgs {
        allow_high_dpi: false,
        borderless: false,
        height: 720,
        width: 1280,
        resizable: true,
        title: "Hello, World"
    })
    .expect("Failed to create Window");

    unsafe {load_global_gl(&|p| _win.get_proc_address(p));}

    'main_loop: loop {
        while let Some(event) = sdl.poll_events() {
            match event {
                (Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }

        unsafe {
            glClearBufferfv(gl33::GL_COLOR, 0, &[0.7_f32, 0.3_f32, 0.3_f32, 1.0_f32] as *const f32);
        }
        _win.swap_window();
    }
}
