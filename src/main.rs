use std::{ffi::CString};

use fermium::{
    prelude::{SDL_Event, SDL_PollEvent, SDL_QUIT},
    video::*,
    *, timer::SDL_GetTicks,
};
use gl33::global_loader::{load_global_gl, glClearBufferfv};


struct GApp {
    window: *mut SDL_Window,
    gl_context: SDL_GLContext,
}

impl GApp {
    fn new(title: &CString) -> GApp {
        unsafe {
            if SDL_Init(SDL_INIT_EVERYTHING) < 0 {
                panic!("SDL init failed");
            }
    
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GL_CONTEXT_PROFILE_CORE.0 as c_int,
            );
        }
        let win = unsafe {
            SDL_CreateWindow(
                title.as_ptr(),
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                800,
                600,
                SDL_WINDOW_OPENGL.0 | SDL_WINDOW_SHOWN.0,
            )
        };
    
        if win.is_null() {
            panic!("Window init failed");
        }
    
        let ctx = unsafe { SDL_GL_CreateContext(win) };
        if ctx.is_null() {
            panic!("GL context failed");
        }
    
        unsafe {
            SDL_GL_SetSwapInterval(1);
            load_global_gl(&|p| SDL_GL_GetProcAddress(p as *const i8));
        }

        GApp { window: win, gl_context: ctx }
    }

    fn destroy(&self) {
        unsafe {
            if !self.gl_context.is_null() {
                SDL_GL_DeleteContext(self.gl_context);
            }
            if !self.window.is_null() {
                SDL_DestroyWindow(self.window);
            }
            SDL_Quit();
        }
    }
}

fn main() {
    let title = CString::new("Blit").unwrap();
    let g_app = GApp::new(&title);

    // main loop
    'mainloop: loop {
        unsafe {
            let mut event = SDL_Event::default();
            while SDL_PollEvent(&mut event) > 0 {
                match event.type_ {
                    SDL_QUIT => {
                        println!("Bye!");
                        break 'mainloop;
                    }
                    _ => {}
                }
            }
            let ticks = SDL_GetTicks() as f64;

            //finally some drawing it only took forever to get here
            // glClearColor(0.7, 0.3, 0.2, 1.0);
            glClearBufferfv(gl33::GL_COLOR, 0, &[(ticks/1000.0).cos().abs() as f32, 0.3_f32, 0.3_f32, 1.0_f32] as *const f32);

            SDL_GL_SwapWindow(g_app.window);
        }
    }

    g_app.destroy();
}
