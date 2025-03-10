use std::{
    ffi::c_void,
    mem::{self, MaybeUninit},
    ptr,
};

use super::{DrawTarget, DrawUsage, Vao};

pub struct Vbo(u32);

impl Vbo {
    pub fn new(vao: &Vao) -> Self {
        unsafe { vao.bind() };
        let mut id: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe { gl::GenBuffers(1, id.as_mut_ptr()) };
        let id = unsafe { id.assume_init() };

        Self(id)
    }

    pub fn bind_data<T>(&self, data: &[T]) {
        self.bind_data_ex(data, DrawTarget::ArrayBuffer, DrawUsage::StaticDraw);
    }

    pub fn bind_data_ex<T>(&self, data: &[T], draw_target: DrawTarget, draw_type: DrawUsage) {
        unsafe {
            gl::BindBuffer(draw_target as u32, self.0);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of_val(data)) as isize,
                data.as_ptr() as *const c_void,
                draw_type as u32,
            )
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, ptr::addr_of!(self.0)) }
    }
}
