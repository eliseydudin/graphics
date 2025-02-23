use crate::{Color, Program, Vao};
use std::{cell::Cell, ffi::c_void, ops};

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

pub struct DrawLayer {
    vao_bound: Cell<bool>,
}

impl DrawLayer {
    pub unsafe fn new<F>(mut loader: F) -> Self
    where
        F: FnMut(&'static str) -> *const (),
    {
        gl::load_with(|s| loader(s) as *const c_void);
        Self {
            vao_bound: Cell::new(false),
        }
    }

    pub fn clear(&self, flags: ClearFlags) {
        unsafe { gl::Clear(flags.0) }
    }

    pub fn set_clear_color(&self, color: Color) {
        unsafe { gl::ClearColor(color.r, color.g, color.b, color.a) }
    }

    pub fn use_program(&self, program: &Program) {
        unsafe { program.use_internal() }
    }

    pub fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        assert!(self.vao_bound.get());
        unsafe { gl::DrawArrays(mode as u32, first, count) }
    }

    pub fn bind(&self, vao: &Vao) {
        unsafe { vao.bind() };
        self.vao_bound.set(true);
    }
}
