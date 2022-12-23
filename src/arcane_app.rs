use gl33::{
    global_loader::{glClearBufferfv, glDrawArrays},
    GL_POINTS,
};

use crate::render_mender::{
    app_behaviour::AppBehaviour, app_state::AppState, shader_program::GLShaderProgram,
    vao::GLVertArrObj,
};

pub struct ArcaneApp {
    shader: GLShaderProgram,
    vao: GLVertArrObj,
}

impl Default for ArcaneApp {
    fn default() -> Self {
        ArcaneApp {
            shader: GLShaderProgram::compile("triangle"),
            vao: GLVertArrObj::new(),
        }
    }
}

impl AppBehaviour for ArcaneApp {
    fn render(&self, app_state: &AppState) {
        //finally some drawing it only took forever to get here
        let red = app_state.ticks().cos().abs() as f32;
        unsafe {
            self.vao.bind();
            self.shader.use_shader();

            glClearBufferfv(
                gl33::GL_COLOR,
                0,
                &[red, 0.3_f32, 0.3_f32, 1.0_f32] as *const f32,
            );

            glDrawArrays(GL_POINTS, 0, 1);

            self.vao.unbind()
        }
    }
    fn update(&self, _app_state: &AppState) {}
}
