use gl33::global_loader::{glBindVertexArray, glDeleteVertexArrays, glGenVertexArrays};

pub struct GLVertArrObj {
    vao: u32,
}

impl GLVertArrObj {
    pub fn new() -> GLVertArrObj {
        unsafe {
            let mut vao = 0;
            glGenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);
            GLVertArrObj { vao: vao }
        }
    }

    pub fn bind(&self) {
        glBindVertexArray(self.vao);
    }
    pub fn unbind(&self) {
        glBindVertexArray(0);
    }
}

impl Drop for GLVertArrObj {
    fn drop(&mut self) {
        unsafe {
            glDeleteVertexArrays(1, &self.vao);
        }
    }
}
