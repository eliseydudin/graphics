use std::{mem::MaybeUninit, ptr};

#[repr(transparent)]
pub struct Vao(u32);

#[allow(unused)]
impl Vao {
    pub fn new() -> Self {
        let mut id: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe { gl::GenVertexArrays(1, id.as_mut_ptr()) };
        let id = unsafe { id.assume_init() };

        Self(id)
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.0)
    }
}

impl Default for Vao {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, ptr::addr_of!(self.0)) }
    }
}
