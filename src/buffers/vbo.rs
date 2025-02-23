use std::{
    ffi::c_void,
    mem::{self, MaybeUninit},
    ptr,
};

use super::Vao;

pub struct Vbo(u32);

#[repr(u32)]
pub enum DrawTarget {
    ArrayBuffer = gl::ARRAY_BUFFER,
    AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
    CopyReadBuffer = gl::COPY_READ_BUFFER,
    CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
    DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
    PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
    PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
    QueryBuffer = gl::QUERY_BUFFER,
    ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
    TextureBuffer = gl::TEXTURE_BUFFER,
    TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
    UniformBuffer = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
pub enum DrawUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

#[allow(unused)]
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
