use gl33::{
    global_loader::{glClearBufferfv, glDrawArrays, glPointSize, glVertexAttrib4fv},
    GL_POINTS, GL_TRIANGLES,
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
        let result = ArcaneApp {
            shader: GLShaderProgram::compile("triangle"),
            vao: GLVertArrObj::new(),
        };

        result
    }
}

impl AppBehaviour for ArcaneApp {
    fn render(&self, app_state: &AppState) {
        //finally some drawing it only took forever to get here
        let red = app_state.ticks().cos().abs() as f32;
        let offset = [
            (app_state.ticks().cos() * 0.5) as f32,
            (app_state.ticks().sin() * 0.6) as f32,
            0.0_f32,
            0.0_f32,
        ];
        unsafe {
            self.vao.bind();
            self.shader.use_shader();

            glClearBufferfv(
                gl33::GL_COLOR,
                0,
                [red, 0.3_f32, 0.3_f32, 1.0_f32].as_ptr().cast()
            );
            glVertexAttrib4fv(0, offset.as_ptr().cast());
            glVertexAttrib4fv(1, offset.as_ptr().cast());
            glDrawArrays(GL_TRIANGLES, 0, 3);

            self.vao.unbind()
        }
    }
    fn update(&self, _app_state: &AppState) {}
}
