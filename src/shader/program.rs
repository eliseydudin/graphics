use crate::Attribute;

use super::{Fragment, Shader, ShaderError, Vertex};
use std::{
    ffi::{c_char, CStr, CString},
    ptr,
};

pub struct Program(u32);

impl Program {
    pub fn new(
        vertex_shader: Shader<Vertex>,
        fragment_shader: Shader<Fragment>,
    ) -> Result<Self, ShaderError> {
        let id = unsafe { Self::link_internal(vertex_shader, fragment_shader) }?;
        Ok(Self(id))
    }

    unsafe fn link_internal(
        vertex_shader: Shader<Vertex>,
        fragment_shader: Shader<Fragment>,
    ) -> Result<u32, ShaderError> {
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex_shader.handle);
        gl::AttachShader(id, fragment_shader.handle);
        gl::LinkProgram(id);

        if !Self::check_link_status(id) {
            return Err(Self::get_error(id));
        }

        Ok(id)
    }

    unsafe fn get_error(id: u32) -> ShaderError {
        let mut buffer = [c_char::MIN; 512];
        gl::GetProgramInfoLog(id, 512, ptr::null_mut(), buffer.as_mut_ptr());

        let err = CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .to_string();

        ShaderError::LinkingError(err)
    }

    unsafe fn check_link_status(id: u32) -> bool {
        let mut success = 1;
        gl::GetProgramiv(id, gl::COMPILE_STATUS, ptr::addr_of_mut!(success));

        success != 0
    }

    pub fn get_attribute(&self, position: &str) -> Option<Attribute> {
        let id = unsafe { self.get_attribute_id(position) }?;

        if id == -1 {
            return None;
        }

        Some(Attribute::at_pos(id as u32))
    }

    unsafe fn get_attribute_id(&self, position: &str) -> Option<i32> {
        let cstr = CString::new(position).ok()?;
        let cstr = cstr.as_ptr();

        let attribute_id = unsafe { gl::GetAttribLocation(self.0, cstr) };
        Some(attribute_id)
    }
}

impl Program {
    pub unsafe fn use_internal(&self) {
        gl::UseProgram(self.0)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) }
    }
}
