use crate::{Color, Program, Vao};
use std::{ffi::c_void, ops};

#[repr(u32)]
pub enum DrawMode {
    /* TODO */
    Triangles = gl::TRIANGLES,
}

pub struct ClearFlags(u32);

impl ClearFlags {
    pub const COLOR: Self = Self(gl::COLOR_BUFFER_BIT);
    pub const DEPTH: Self = Self(gl::DEPTH_BUFFER_BIT);
    pub const STENCIL: Self = Self(gl::STENCIL_BUFFER_BIT);
}

impl ops::BitOr for ClearFlags {
    type Output = u32;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.0 | rhs.0
    }
}

pub struct DrawLayer;

impl DrawLayer {
    /// Initialize OpenGL and create a [`DrawLayer`].
    /// `loader` is the function which will be used to initialize OpenGL.
    pub unsafe fn new<T, F>(mut loader: F) -> Self
    where
        F: FnMut(&'static str) -> *const T,
    {
        gl::load_with(|s| loader(s) as *const c_void);
        Self
    }

    /// Clears the screen.
    pub fn clear(&self, flags: ClearFlags) {
        unsafe { gl::Clear(flags.0) }
    }

    /// Set the color which will be used when [`Self::clear`] is called
    pub fn set_clear_color(&self, color: Color) {
        unsafe { gl::ClearColor(color.r, color.g, color.b, color.a) }
    }

    /// Use a shader program
    pub fn use_program(&self, program: &Program) {
        unsafe { program.use_internal() }
    }

    /// Draw the information behind [`Vao`] to the screen
    pub fn draw_arrays(&self, vao: &Vao, mode: DrawMode, first: i32, count: i32) {
        unsafe {
            vao.bind();
            gl::DrawArrays(mode as u32, first, count)
        }
    }
}
