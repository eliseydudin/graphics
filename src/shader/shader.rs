use std::{
    ffi::{c_char, CStr, CString},
    marker::PhantomData,
    ptr,
};

use super::ShaderError;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    Fragment = gl::FRAGMENT_SHADER,
    Vertex = gl::VERTEX_SHADER,
}

pub trait AsShaderType {
    fn as_shader_type() -> ShaderType;
}

pub struct Fragment;

impl AsShaderType for Fragment {
    fn as_shader_type() -> ShaderType {
        ShaderType::Fragment
    }
}

pub struct Vertex;

impl AsShaderType for Vertex {
    fn as_shader_type() -> ShaderType {
        ShaderType::Vertex
    }
}

pub struct Shader<S: AsShaderType> {
    pub(crate) handle: u32,
    data: PhantomData<S>,
}

impl<S: AsShaderType> Shader<S> {
    pub fn compile(source: &str) -> Result<Self, ShaderError> {
        let cstr = CString::new(source).map_err(|e| ShaderError::CStringConversion(e))?;
        let ptr = cstr.as_ptr();

        let id = unsafe { Self::compile_internal(ptr) }?;
        Ok(Self {
            handle: id,
            data: PhantomData,
        })
    }

    unsafe fn compile_internal(source: *const c_char) -> Result<u32, ShaderError> {
        let id = gl::CreateShader(S::as_shader_type() as u32);
        gl::ShaderSource(id, 1, ptr::addr_of!(source), ptr::null());
        gl::CompileShader(id);

        if !Self::check_compile_status(id) {
            return Err(Self::get_error(id));
        }

        Ok(id)
    }

    unsafe fn get_error(id: u32) -> ShaderError {
        let mut buffer = [c_char::MIN; 512];
        gl::GetShaderInfoLog(id, 512, ptr::null_mut(), buffer.as_mut_ptr());

        let err = CStr::from_ptr(buffer.as_ptr())
            .to_string_lossy()
            .to_string();

        ShaderError::CompilationError(err)
    }

    unsafe fn check_compile_status(id: u32) -> bool {
        let mut success = 1;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, ptr::addr_of_mut!(success));

        success != 0
    }
}

impl<S: AsShaderType> Drop for Shader<S> {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle) }
    }
}
