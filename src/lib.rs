#![doc = include_str!("../README.md")]
use std::{
    ffi::{CStr, OsStr},
    mem::MaybeUninit,
    sync::Arc,
};

#[allow(
    non_upper_case_globals,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::too_many_arguments,
    clippy::enum_variant_names,
    clippy::missing_safety_doc
)]
pub mod bindings;
pub use bindings as ffi;

pub struct IRShaderReflection {
    me: *mut bindings::IRShaderReflection,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRShaderReflection {
    fn drop(&mut self) {
        unsafe { self.funcs.IRShaderReflectionDestroy(self.me) }
    }
}

impl IRShaderReflection {
    pub fn new(compiler: &IRCompiler) -> Result<IRShaderReflection, Box<dyn std::error::Error>> {
        unsafe {
            let me = compiler.funcs.IRShaderReflectionCreate();
            Ok(Self {
                me,
                funcs: compiler.funcs.clone(),
            })
        }
    }

    pub fn get_compute_info(
        &self,
        version: ffi::IRReflectionVersion,
    ) -> Result<ffi::IRVersionedCSInfo, Box<dyn std::error::Error>> {
        let mut info = MaybeUninit::uninit();
        if unsafe {
            self.funcs
                .IRShaderReflectionCopyComputeInfo(self.me, version, info.as_mut_ptr())
        } {
            Ok(unsafe { info.assume_init() })
        } else {
            panic!("Test me");
        }
    }
}

pub struct IRObject {
    me: *mut bindings::IRObject,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRObject {
    fn drop(&mut self) {
        unsafe { self.funcs.IRObjectDestroy(self.me) }
    }
}

impl IRObject {
    pub fn create_from_dxil(
        compiler: &IRCompiler,
        bytecode: &[u8],
    ) -> Result<IRObject, Box<dyn std::error::Error>> {
        unsafe {
            let me = compiler.funcs.IRObjectCreateFromDXIL(
                bytecode.as_ptr(),
                bytecode.len(),
                bindings::IRBytecodeOwnership::None,
            );

            Ok(Self {
                me,
                funcs: compiler.funcs.clone(),
            })
        }
    }

    pub fn get_type(&self) -> ffi::IRObjectType {
        unsafe { self.funcs.IRObjectGetType(self.me) }
    }

    pub fn get_metal_ir_shader_stage(&self) -> ffi::IRShaderStage {
        unsafe { self.funcs.IRObjectGetMetalIRShaderStage(self.me) }
    }

    pub fn get_metal_lib_binary(
        &self,
        shader_stage: ffi::IRShaderStage,
        dest_lib: &mut IRMetalLibBinary,
    ) -> bool {
        unsafe {
            self.funcs
                .IRObjectGetMetalLibBinary(self.me, shader_stage, dest_lib.me)
        }
    }

    pub fn get_reflection(
        &self,
        shader_stage: ffi::IRShaderStage,
        reflection: &mut IRShaderReflection,
    ) -> bool {
        unsafe {
            self.funcs
                .IRObjectGetReflection(self.me, shader_stage, reflection.me)
        }
    }
}

pub struct IRMetalLibBinary {
    me: *mut bindings::IRMetalLibBinary,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRMetalLibBinary {
    fn drop(&mut self) {
        unsafe { self.funcs.IRMetalLibBinaryDestroy(self.me) }
    }
}

impl IRMetalLibBinary {
    pub fn new(compiler: &IRCompiler) -> Result<IRMetalLibBinary, Box<dyn std::error::Error>> {
        unsafe {
            let me = compiler.funcs.IRMetalLibBinaryCreate();
            Ok(Self {
                me,
                funcs: compiler.funcs.clone(),
            })
        }
    }

    pub fn get_byte_code(&self) -> Vec<u8> {
        let size_in_bytes = unsafe { self.funcs.IRMetalLibGetBytecodeSize(self.me) };
        let mut bytes = vec![0u8; size_in_bytes];
        let _written = unsafe {
            self.funcs
                .IRMetalLibGetBytecode(self.me, bytes.as_mut_ptr())
        };
        bytes
    }
}

pub struct IRRootSignature {
    me: *mut bindings::IRRootSignature,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRRootSignature {
    fn drop(&mut self) {
        unsafe { self.funcs.IRRootSignatureDestroy(self.me) }
    }
}

impl IRRootSignature {
    pub fn create_from_descriptor(
        compiler: &IRCompiler,
        desc: &ffi::IRVersionedRootSignatureDescriptor,
    ) -> Result<IRRootSignature, Box<dyn std::error::Error>> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let me = compiler
                .funcs
                .IRRootSignatureCreateFromDescriptor(desc, &mut error);

            Ok(Self {
                me,
                funcs: compiler.funcs.clone(),
            })
        }
    }
}

/// [`IRCompilerFactory`] is used to load the `metal_irconverter` dynamic library and holds its functions in an [`Arc`].
/// Since [`IRCompiler`] is not thread-safe, this struct provides an interface to create [`IRCompiler`] instances.
/// This way, the library only has to be loaded once, but each thread can have its own [`IRCompiler`] instance.
pub struct IRCompilerFactory {
    funcs: Arc<bindings::metal_irconverter>,
}

impl IRCompilerFactory {
    pub fn new(lib_path: impl AsRef<OsStr>) -> Result<Self, libloading::Error> {
        let funcs = Arc::new(unsafe { bindings::metal_irconverter::new(lib_path)? });
        Ok(Self { funcs })
    }

    pub fn from_library(lib: libloading::Library) -> Result<Self, libloading::Error> {
        let funcs = Arc::new(unsafe { bindings::metal_irconverter::from_library(lib)? });
        Ok(Self { funcs })
    }

    pub fn create_compiler(&self) -> IRCompiler {
        let compiler = unsafe { self.funcs.IRCompilerCreate() };
        IRCompiler {
            me: compiler,
            funcs: self.funcs.clone(),
        }
    }
}

/// This object is not thread-safe, refer to [the Metal shader converter documentation], the "Multithreading considerations" chapter.
///
/// [the Metal shader converter documentation]: https://developer.apple.com/metal/shader-converter/
pub struct IRCompiler {
    me: *mut bindings::IRCompiler,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRCompiler {
    fn drop(&mut self) {
        unsafe { self.funcs.IRCompilerDestroy(self.me) }
    }
}

impl IRCompiler {
    pub fn set_global_root_signature(&mut self, root_signature: &IRRootSignature) {
        unsafe {
            self.funcs
                .IRCompilerSetGlobalRootSignature(self.me, root_signature.me)
        }
    }

    pub fn set_entry_point_name(&mut self, new_name: &CStr) {
        unsafe {
            self.funcs
                .IRCompilerSetEntryPointName(self.me, new_name.as_ptr())
        }
    }

    pub fn alloc_compile_and_link(
        &mut self,
        entry_point: &CStr,
        input: &IRObject,
    ) -> Result<IRObject, Box<dyn std::error::Error>> {
        let mut error = std::ptr::null_mut();

        let v = unsafe {
            self.funcs.IRCompilerAllocCompileAndLink(
                self.me,
                entry_point.as_ptr(),
                input.me,
                &mut error,
            )
        };

        if error.is_null() {
            Ok(IRObject {
                me: v,
                funcs: input.funcs.clone(),
            })
        } else {
            panic!("{:?}", error);
        }
    }
}
