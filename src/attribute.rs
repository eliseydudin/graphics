use std::ffi::c_void;

#[repr(u32)]
#[allow(unused)]
pub enum AttributeType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    Float = gl::FLOAT,
    HalfFloat = gl::HALF_FLOAT,
    Double = gl::DOUBLE,
    Fixed = gl::FILL,
}

pub struct Attribute(u32);

impl Attribute {
    pub(crate) const fn at_pos(id: u32) -> Self {
        Self(id)
    }

    pub fn enable(&self) -> &Self {
        unsafe { gl::EnableVertexAttribArray(self.0) };
        self
    }

    pub fn memory_layout(
        &self,
        size: i32,
        mem_type: AttributeType,
        normalized: bool,
        stride: usize,
        offset: usize,
    ) -> &Self {
        unsafe {
            gl::VertexAttribPointer(
                self.0,
                size,
                mem_type as u32,
                normalized as u8,
                stride as i32,
                offset as *const c_void,
            )
        }

        self
    }
}
