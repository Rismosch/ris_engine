use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr::NonNull;

use crate::bindings::shaderc;

pub struct Compiler {
    boo: PhantomData<shaderc::shaderc_compiler>,
    inner: NonNull<shaderc::shaderc_compiler>,
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            shaderc::shaderc_compiler_release(self.inner.as_ptr())
        };
    }
}



impl Compiler {
    pub fn initialize() -> Option<Self> {
        let ptr = unsafe{shaderc::shaderc_compiler_initialize()};
        let inner = NonNull::new(ptr)?;

        Some(Self {
            boo: PhantomData,
            inner,
        })
    }
}

pub struct CompileOptions {
    boo: PhantomData<shaderc::shaderc_compile_options>,
    inner: NonNull<shaderc::shaderc_compile_options>,
}

impl Drop for CompileOptions {
    fn drop(&mut self) {
        unsafe{shaderc::shaderc_compile_options_release(self.inner.as_ptr())};
    }
}

impl CompileOptions {
    pub fn initialize() -> Option<Self> {
        let ptr = unsafe{shaderc::shaderc_compile_options_initialize()};
        let inner = NonNull::new(ptr)?;

        Some(Self{
            boo: PhantomData,
            inner,
        })
    }
    
    pub fn clone(&self) -> Option<Self> {
        let ptr = unsafe{shaderc::shaderc_compile_options_clone(self.inner.as_ptr())};
        let inner = NonNull::new(ptr)?;

        Some(Self{
            boo: PhantomData,
            inner,
        })
    }

    pub fn add_macro_definition(&mut self, name: impl AsRef<str>, value: Option<impl AsRef<str>>) {
        let name = name.as_ref();
        let value = value.as_ref().map(|x| x.as_ref());

        let name_ptr = name.as_ptr() as *const c_char;
        let name_length = name.len();

        let (value_ptr, value_length) = match value {
            Some(value) => (value.as_ptr() as *const c_char, value.len()),
            None => (std::ptr::null(), 0)
        };

        unsafe{shaderc::shaderc_compile_options_add_macro_definition(
            self.inner.as_ptr(),
            name_ptr,
            name_length,
            value_ptr,
            value_length,
        )};
    }
}

