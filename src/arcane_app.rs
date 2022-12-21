use gl33::global_loader::glClearBufferfv;

use crate::render_mender::{app_state::AppState, app_behaviour::AppBehaviour};

pub struct ArcaneApp {}

impl Default for ArcaneApp {
    fn default() -> Self {
        ArcaneApp {}
    }
}

impl AppBehaviour for ArcaneApp {
    fn render(&self, app_state: &AppState) {
        //finally some drawing it only took forever to get here
        let red = app_state.ticks().cos().abs() as f32;
        unsafe {
            glClearBufferfv(
                gl33::GL_COLOR,
                0,
                &[red, 0.3_f32, 0.3_f32, 1.0_f32] as *const f32,
            );
        }
    }
    fn update(&self, app_state: &AppState) {}
}
