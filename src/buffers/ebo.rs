use super::{DrawTarget, DrawUsage, Vao};
use std::{ffi, mem, ptr};

pub struct Ebo(u32);

impl Ebo {
    pub fn new(vao: &Vao) -> Self {
        unsafe { vao.bind() };
        let mut id = 0;
        unsafe { gl::GenBuffers(1, ptr::addr_of_mut!(id)) };

        Self(id)
    }

    pub fn bind_data<T>(&self, data: &[T]) {
        self.bind_data_ex(data, DrawTarget::ElementArrayBuffer, DrawUsage::StaticDraw);
    }

    pub fn bind_data_ex<T>(&self, data: &[T], draw_target: DrawTarget, draw_type: DrawUsage) {
        unsafe {
            gl::BindBuffer(draw_target as u32, self.0);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of_val(data)) as isize,
                data.as_ptr() as *const ffi::c_void,
                draw_type as u32,
            )
        }
    }
}

impl Drop for Ebo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, ptr::addr_of!(self.0)) }
    }
}
