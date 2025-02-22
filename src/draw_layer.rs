use crate::{Color, Program, Vao};
use std::{cell::Cell, ffi::c_void, ops};

#[repr(u32)]
pub enum DrawMode {
    /* TODO */
    Triangles = gl::TRIANGLES,
}

#[repr(u32)]
pub enum ClearFlags {
    Color = gl::COLOR_BUFFER_BIT,
    Depth = gl::DEPTH_BUFFER_BIT,
    Stencil = gl::STENCIL_BUFFER_BIT,
}

impl ops::BitOr for ClearFlags {
    type Output = u32;
    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl Into<u32> for ClearFlags {
    fn into(self) -> u32 {
        self as u32
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

    pub fn clear(&self, flags: impl Into<u32>) {
        unsafe { gl::Clear(flags.into()) }
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
