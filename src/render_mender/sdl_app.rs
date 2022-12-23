use std::ffi::CString;

use fermium::{
    c_int,
    prelude::{SDL_Event, SDL_PollEvent, SDL_QUIT},
    video::*,
    SDL_Init, SDL_Quit, SDL_INIT_EVERYTHING,
};
use gl33::global_loader::load_global_gl;

use super::{app_behaviour::AppBehaviour, app_state::AppState};

pub struct SdlApp {
    pub window: *mut SDL_Window,
    gl_context: SDL_GLContext,
    app_state: AppState,
}

#[derive(PartialEq, Clone, Copy)]
enum QuitApp {
    Quit = 0,
    Continue = 1,
}

impl SdlApp {
    pub fn new(title: &CString) -> SdlApp {
        unsafe {
            assert!(SDL_Init(SDL_INIT_EVERYTHING) == 0);

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
        assert!(!win.is_null());

        let ctx = unsafe { SDL_GL_CreateContext(win) };
        assert!(!ctx.is_null());

        unsafe {
            SDL_GL_SetSwapInterval(1);
            load_global_gl(&|p| SDL_GL_GetProcAddress(p as *const i8));
        }

        SdlApp {
            window: win,
            gl_context: ctx,
            app_state: AppState::new(),
        }
    }

    pub fn main_loop(&mut self, app_behav: &mut impl AppBehaviour) {
        'mainloop: loop {
            unsafe {
                if self.poll_events() == QuitApp::Quit {
                    break 'mainloop;
                }
                // TODO implement proper fixed step
                self.app_state.update_ticks();

                app_behav.update(&self.app_state);
                app_behav.render(&self.app_state);

                SDL_GL_SwapWindow(self.window);
            }
        }
    }

    fn poll_events(&mut self) -> QuitApp {
        unsafe {
            let mut event = SDL_Event::default();
            while SDL_PollEvent(&mut event) > 0 {
                match event.type_ {
                    SDL_QUIT => {
                        return QuitApp::Quit;
                    }
                    _ => {}
                }
            }
        }

        QuitApp::Continue
    }
}

impl Drop for SdlApp {
    fn drop(&mut self) {
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
