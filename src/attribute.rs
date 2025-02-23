use std::ffi::c_void;

use crate::Program;

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum AttributeType {
    i8 = gl::BYTE,
    u8 = gl::UNSIGNED_BYTE,
    i16 = gl::SHORT,
    u16 = gl::UNSIGNED_SHORT,
    f32 = gl::FLOAT,
    f16 = gl::HALF_FLOAT,
    f64 = gl::DOUBLE,
    i32 = gl::INT,
    u32 = gl::UNSIGNED_INT,
    fixed = gl::FIXED,
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
        stride: i32,
        offset: usize,
    ) -> &Self {
        unsafe {
            gl::VertexAttribPointer(
                self.0,
                size,
                mem_type as u32,
                normalized as u8,
                stride,
                offset as *const c_void,
            )
        }

        self
    }
}

pub struct AttributeDescriptor {
    pub name: &'static str,
    pub vector_size: i32,
    pub mem_size: usize,
    pub mem_type: AttributeType,
}

impl AttributeDescriptor {
    pub fn new(
        name: &'static str,
        vector_size: i32,
        mem_size: usize,
        mem_type: AttributeType,
    ) -> Self {
        Self {
            name,
            vector_size,
            mem_size,
            mem_type,
        }
    }
}

#[macro_export]
macro_rules! attributes {
    ($name:ident: vec<$tp:ident, $size:literal>) => {
        $crate::AttributeDescriptor::new(stringify!($name), $size, core::mem::size_of::<$tp>(), $crate::AttributeType::$tp)
    };

    ($name:ident: vec<$tp:ident, $size:literal>, $($name1:ident: vec<$tp1:ident, $size1:literal>), +) => {
        $crate::Attributes::new(vec![
            attributes!($name: vec<$tp, $size>),
            attributes!($($name1: vec<$tp1, $size1>), +)
        ])

    };
}

pub struct Attributes {
    attrs: Vec<AttributeDescriptor>,
}

impl Attributes {
    pub fn new(attrs: Vec<AttributeDescriptor>) -> Self {
        Self { attrs }
    }

    pub fn calculate_for(self, program: &Program) -> Option<()> {
        let mut stride = 0;
        for attr in &self.attrs {
            stride += attr.vector_size * attr.mem_size as i32;
        }

        {
            let first_attr = &self.attrs[0];

            let attr = program.get_attribute(&first_attr.name)?;
            attr.enable();
            attr.memory_layout(
                first_attr.vector_size,
                first_attr.mem_type,
                false,
                stride,
                0,
            );
        }

        let mut offset = 0;
        for attr_pair in self.attrs.windows(2) {
            offset += attr_pair[0].mem_size * attr_pair[0].vector_size as usize;

            let second = &attr_pair[1];
            let attr = program.get_attribute(second.name)?;
            attr.enable();
            attr.memory_layout(second.vector_size, second.mem_type, false, stride, offset);
        }

        Some(())
    }
}
