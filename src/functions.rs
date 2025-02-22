use std::ops;

use crate::Color;

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

pub fn clear(flags: impl Into<u32>) {
    unsafe { gl::Clear(flags.into()) }
}

pub fn set_clear_color(color: Color) {
    unsafe { gl::ClearColor(color.r, color.g, color.b, color.a) }
}
