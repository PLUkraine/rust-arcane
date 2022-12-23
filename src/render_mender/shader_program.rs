use std::{fs::read_to_string, path::PathBuf};

use fermium::c_uint;
use gl33::{
    global_loader::{
        glAttachShader, glCompileShader, glCreateProgram, glCreateShader, glDeleteProgram,
        glDeleteShader, glGetProgramInfoLog, glGetProgramiv, glGetShaderInfoLog, glGetShaderiv,
        glLinkProgram, glShaderSource, glUseProgram,
    },
    GL_COMPILE_STATUS, GL_FRAGMENT_SHADER, GL_LINK_STATUS, GL_VERTEX_SHADER,
};

fn read_shader(shader_name: &str) -> String {
    let mut shader_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    shader_path.push("shaders");
    shader_path.push(shader_name);

    let ppska = shader_path.to_str().expect("Why are we still here just to suffer");
    println!("Reading {}", ppska);
    read_to_string(shader_path).expect("Shader read failed")
}

pub struct GLShaderProgram {
    program: c_uint,
}

impl GLShaderProgram {
    fn assert_shader_compiled(shader_handle: u32) {
        unsafe {
            let mut success = 0;
            glGetShaderiv(shader_handle, GL_COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                glGetShaderInfoLog(shader_handle, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }
    fn assert_shader_linked(shader_program: u32) {
        unsafe {
            let mut success = 0;
            glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                glGetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }

    pub fn compile(material: &str) -> GLShaderProgram {
        let vert_path = [material, ".vert.glsl"].join("");
        let frag_path = [material, ".frag.glsl"].join("");
        let vertex_code = read_shader(&vert_path);
        let fragment_code = read_shader(&frag_path);

        unsafe {
            let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
            glShaderSource(
                vertex_shader,
                1,
                &(vertex_code.as_bytes().as_ptr().cast()),
                &(vertex_code.len().try_into().unwrap()),
            );
            glCompileShader(vertex_shader);
            GLShaderProgram::assert_shader_compiled(vertex_shader);

            let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
            glShaderSource(
                fragment_shader,
                1,
                &(fragment_code.as_bytes().as_ptr().cast()),
                &(fragment_code.len().try_into().unwrap()),
            );
            glCompileShader(fragment_shader);
            GLShaderProgram::assert_shader_compiled(fragment_shader);

            let program = glCreateProgram();
            glAttachShader(program, vertex_shader);
            glAttachShader(program, fragment_shader);
            glLinkProgram(program);
            GLShaderProgram::assert_shader_linked(program);

            glDeleteShader(vertex_shader);
            glDeleteShader(fragment_shader);

            GLShaderProgram { program: program }
        }
    }

    pub fn use_shader(&self) {
        glUseProgram(self.program);
    }
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        glDeleteProgram(self.program);
    }
}
