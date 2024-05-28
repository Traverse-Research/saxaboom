#![doc = include_str!("../README.md")]
#![deny(clippy::use_self, elided_lifetimes_in_paths)]
use std::{
    ffi::{CStr, CString, OsStr},
    mem::MaybeUninit,
    ptr::NonNull,
};

#[allow(
    clippy::enum_variant_names,
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::use_self,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod bindings;
pub use bindings as ffi;
use ffi::IRErrorCode;
use thiserror::Error;

pub struct IRShaderReflection<'lib> {
    me: NonNull<bindings::IRShaderReflection>,
    funcs: &'lib bindings::metal_irconverter,
}

impl<'lib> Drop for IRShaderReflection<'lib> {
    #[doc(alias = "IRShaderReflectionDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRShaderReflectionDestroy(self.me.as_ptr()) }
    }
}

impl<'lib> IRShaderReflection<'lib> {
    #[doc(alias = "IRShaderReflectionCreate")]
    pub fn new(compiler: &'lib IRCompiler<'lib>) -> Self {
        let me = NonNull::new(unsafe { compiler.funcs.IRShaderReflectionCreate() })
            .expect("Failed to create IRShaderReflection");
        Self {
            me,
            funcs: compiler.funcs,
        }
    }

    #[doc(alias = "IRShaderReflectionCopyComputeInfo")]
    pub fn compute_info(
        &self,
        version: ffi::IRReflectionVersion,
    ) -> Option<ffi::IRVersionedCSInfo> {
        let mut info = MaybeUninit::uninit();
        if unsafe {
            self.funcs.IRShaderReflectionCopyComputeInfo(
                self.me.as_ptr(),
                version,
                info.as_mut_ptr(),
            )
        } {
            Some(unsafe { info.assume_init() })
        } else {
            None
        }
    }
}

#[derive(Error, Debug)]
#[error("Failed to get MetalLib bytecode from IRObject")]
pub struct MetalLibNoBytecodeFoundError(ffi::IRObjectType, ffi::IRShaderStage);

pub struct IRObject<'lib> {
    me: NonNull<bindings::IRObject>,
    funcs: &'lib bindings::metal_irconverter,
    compiler: &'lib IRCompiler<'lib>,
}

impl<'lib> Drop for IRObject<'lib> {
    #[doc(alias = "IRObjectDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRObjectDestroy(self.me.as_ptr()) }
    }
}

impl<'lib> IRObject<'lib> {
    #[doc(alias = "IRObjectCreateFromDXIL")]
    pub fn create_from_dxil(compiler: &'lib IRCompiler<'lib>, bytecode: &[u8]) -> IRObject<'lib> {
        unsafe {
            let me = NonNull::new(compiler.funcs.IRObjectCreateFromDXIL(
                bytecode.as_ptr(),
                bytecode.len(),
                bindings::IRBytecodeOwnership::Copy,
            ))
            .expect("Failed to create IRObject from DXIL");

            Self {
                me,
                funcs: compiler.funcs,
                compiler,
            }
        }
    }

    #[doc(alias = "IRObjectGatherRaytracingIntrinsics")]
    pub fn gather_raytracing_intrinsics(&self, entry_point: &CStr) -> u64 {
        unsafe {
            self.funcs
                .IRObjectGatherRaytracingIntrinsics(self.me.as_ptr(), entry_point.as_ptr())
        }
    }

    #[doc(alias = "IRObjectGetType")]
    pub fn object_type(&self) -> ffi::IRObjectType {
        unsafe { self.funcs.IRObjectGetType(self.me.as_ptr()) }
    }

    #[doc(alias = "IRObjectGetMetalIRShaderStage")]
    pub fn metal_ir_shader_stage(&self) -> ffi::IRShaderStage {
        unsafe { self.funcs.IRObjectGetMetalIRShaderStage(self.me.as_ptr()) }
    }

    #[doc(alias = "IRObjectGetMetalLibBinary")]
    pub fn metal_lib_binary(
        &self,
        shader_stage: ffi::IRShaderStage,
    ) -> Result<IRMetalLibBinary<'lib>, MetalLibNoBytecodeFoundError> {
        let mtl_lib = IRMetalLibBinary::new(self.compiler);
        if unsafe {
            self.funcs.IRObjectGetMetalLibBinary(
                self.me.as_ptr(),
                shader_stage,
                mtl_lib.me.as_ptr(),
            )
        } {
            Ok(mtl_lib)
        } else {
            let obj_type = unsafe { self.funcs.IRObjectGetType(self.me.as_ptr()) };
            let stage = unsafe { self.funcs.IRObjectGetMetalIRShaderStage(self.me.as_ptr()) };
            Err(MetalLibNoBytecodeFoundError(obj_type, stage))
        }
    }

    #[doc(alias = "IRObjectGetReflection")]
    pub fn reflection(&self, shader_stage: ffi::IRShaderStage) -> Option<IRShaderReflection<'lib>> {
        let reflection = IRShaderReflection::new(self.compiler);
        if unsafe {
            self.funcs
                .IRObjectGetReflection(self.me.as_ptr(), shader_stage, reflection.me.as_ptr())
        } {
            Some(reflection)
        } else {
            None
        }
    }
}

pub struct IRMetalLibBinary <'lib>{
    me: NonNull<bindings::IRMetalLibBinary>,
    funcs: &'lib bindings::metal_irconverter,
}

impl<'lib> Drop for IRMetalLibBinary<'lib> {
    #[doc(alias = "IRMetalLibBinaryDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRMetalLibBinaryDestroy(self.me.as_ptr()) }
    }
}

impl <'lib>IRMetalLibBinary<'lib> {
    #[doc(alias = "IRMetalLibBinaryCreate")]
    pub fn new(compiler: &IRCompiler<'lib>) -> Self {
        unsafe {
            let me = NonNull::new(compiler.funcs.IRMetalLibBinaryCreate())
                .expect("Failed to create empty IRMetalLibBinary");
            Self {
                me,
                funcs: compiler.funcs,
            }
        }
    }

    #[doc(alias("IRMetalLibGetBytecode", "IRMetalLibGetBytecodeSize"))]
    pub fn byte_code(&self) -> Vec<u8> {
        let size_in_bytes = unsafe { self.funcs.IRMetalLibGetBytecodeSize(self.me.as_ptr()) };
        let mut bytes = Vec::with_capacity(size_in_bytes);
        let written = unsafe {
            self.funcs
                .IRMetalLibGetBytecode(self.me.as_ptr(), bytes.as_mut_ptr())
        };
        unsafe { bytes.set_len(written) }
        assert_eq!(written, size_in_bytes);
        bytes
    }
}

pub struct IRRootSignature<'lib> {
    me: NonNull<bindings::IRRootSignature>,
    funcs: &'lib bindings::metal_irconverter,
}

impl<'lib> Drop for IRRootSignature<'lib> {
    #[doc(alias = "IRRootSignatureDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRRootSignatureDestroy(self.me.as_ptr()) }
    }
}

#[derive(Error, Debug)]
pub enum RootSignatureError {
    #[error("Failed to create IRRootSignature: {0:?}")]
    CreateError(ffi::IRErrorCode),
    #[error("Failed to create IRRootSignature")]
    Unknown,
}

impl<'lib> IRRootSignature <'lib>{
    #[doc(alias = "IRRootSignatureCreateFromDescriptor")]
    pub fn create_from_descriptor(
        compiler: &'lib IRCompiler<'lib>,
        desc: &ffi::IRVersionedRootSignatureDescriptor,
    ) -> Result<Self, RootSignatureError> {
        let mut error = std::ptr::null_mut();

        let me = NonNull::new(unsafe {
            compiler
                .funcs
                .IRRootSignatureCreateFromDescriptor(desc, &mut error)
        });

        // If the root signature failed to create
        if !error.is_null() {
            // IRErrorCode is #[repr(u32)], so this transmute should be fine
            let error = IRError::from_ptr(error, compiler.funcs);
            Err(RootSignatureError::CreateError(error.code()))
        } else {
            match me {
                Some(me) => Ok(Self {
                    me,
                    funcs: compiler.funcs,
                }),
                None => Err(RootSignatureError::Unknown),
            }
        }
    }

    #[doc(alias(
        "IRRootSignatureGetResourceCount",
        "IRRootSignatureGetResourceLocations"
    ))]
    pub fn resource_locations(&self) -> Vec<ffi::IRResourceLocation> {
        let n_resources = unsafe { self.funcs.IRRootSignatureGetResourceCount(self.me.as_ptr()) };
        let empty_location = ffi::IRResourceLocation {
            resourceType: ffi::IRResourceType::Invalid,
            space: 0,
            slot: 0,
            topLevelOffset: 0,
            sizeBytes: 0,
            resourceName: std::ptr::null(),
        };
        let mut resource_locations = vec![empty_location; n_resources];
        unsafe {
            self.funcs.IRRootSignatureGetResourceLocations(
                self.me.as_ptr(),
                resource_locations.as_mut_ptr(),
            )
        };
        resource_locations
    }
}

/// [`IRCompilerFactory`] is used to load the `metal_irconverter` dynamic library and holds its functions in an [`Arc`].
/// Since [`IRCompiler`] is not thread-safe, this struct provides an interface to create [`IRCompiler`] instances.
/// This way, the library only has to be loaded once, but each thread can have its own [`IRCompiler`] instance.
pub struct IRCompilerFactory {
    funcs: bindings::metal_irconverter,
}

impl IRCompilerFactory {
    pub fn new(lib_path: impl AsRef<OsStr>) -> Result<Self, libloading::Error> {
        let funcs = unsafe { bindings::metal_irconverter::new(lib_path)? };
        Ok(Self { funcs })
    }

    pub fn from_library(lib: libloading::Library) -> Result<Self, libloading::Error> {
        let funcs = unsafe { bindings::metal_irconverter::from_library(lib)? };
        Ok(Self { funcs })
    }

    #[doc(alias = "IRCompilerCreate")]
    pub fn create_compiler(&self) -> IRCompiler<'_> {
        let compiler = NonNull::new(unsafe { self.funcs.IRCompilerCreate() })
            .expect("Failed to create IRCompiler");
        IRCompiler {
            me: compiler,
            funcs: &self.funcs,
        }
    }
}

#[derive(Error, Debug)]
#[error("Failed to compile IRObject: ({0:?})")]
pub struct CompilerError(ffi::IRErrorCode);

/// This object is not thread-safe, refer to [the Metal shader converter documentation], the "Multithreading considerations" chapter.
///
/// [the Metal shader converter documentation]: https://developer.apple.com/metal/shader-converter/
pub struct IRCompiler<'lib> {
    me: NonNull<bindings::IRCompiler>,
    funcs: &'lib bindings::metal_irconverter,
}

impl <'lib>Drop for IRCompiler <'lib>{
    #[doc(alias = "IRCompilerDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRCompilerDestroy(self.me.as_ptr()) }
    }
}

impl<'lib> IRCompiler<'lib> {
    #[doc(alias = "IRCompilerSetGlobalRootSignature")]
    pub fn set_global_root_signature(&mut self, root_signature: &IRRootSignature<'_>) {
        unsafe {
            self.funcs
                .IRCompilerSetGlobalRootSignature(self.me.as_ptr(), root_signature.me.as_ptr())
        }
    }

    #[doc(alias = "IRCompilerSetRayTracingPipelineArguments")]
    #[allow(clippy::too_many_arguments)]
    pub fn set_ray_tracing_pipeline_arguments(
        &mut self,
        max_attribute_size_in_bytes: u32,
        raytracing_pipeline_flags: ffi::IRRaytracingPipelineFlags,
        chs: u64,
        miss: u64,
        any_hit: u64,
        callable_args: u64,
        max_recursive_depth: i32,
    ) {
        unsafe {
            self.funcs.IRCompilerSetRayTracingPipelineArguments(
                self.me.as_ptr(),
                max_attribute_size_in_bytes,
                raytracing_pipeline_flags,
                chs,
                miss,
                any_hit,
                callable_args,
                max_recursive_depth,
            )
        }
    }

    #[doc(alias = "IRCompilerSetHitgroupType")]
    pub fn set_hitgroup_type(&mut self, hit_group_type: ffi::IRHitGroupType) {
        unsafe {
            self.funcs
                .IRCompilerSetHitgroupType(self.me.as_ptr(), hit_group_type)
        }
    }

    #[doc(alias = "IRMetalLibSynthesizeIndirectIntersectionFunction")]
    pub fn synthesize_indirect_intersection_function(&mut self) -> Option<IRMetalLibBinary<'lib>> {
        let target_metallib = IRMetalLibBinary::new(self);
        if unsafe {
            self.funcs.IRMetalLibSynthesizeIndirectIntersectionFunction(
                self.me.as_ptr(),
                target_metallib.me.as_ptr(),
            )
        } {
            Some(target_metallib)
        } else {
            None
        }
    }

    #[doc(alias = "IRMetalLibSynthesizeIndirectRayDispatchFunction")]
    pub fn synthesize_indirect_ray_dispatch_function(&mut self) -> Option<IRMetalLibBinary<'lib>> {
        let target_metallib = IRMetalLibBinary::new(self);
        if unsafe {
            self.funcs.IRMetalLibSynthesizeIndirectRayDispatchFunction(
                self.me.as_ptr(),
                target_metallib.me.as_ptr(),
            )
        } {
            Some(target_metallib)
        } else {
            None
        }
    }

    #[doc(alias = "IRCompilerSetEntryPointName")]
    pub fn set_entry_point_name(&mut self, new_name: &CStr) {
        unsafe {
            self.funcs
                .IRCompilerSetEntryPointName(self.me.as_ptr(), new_name.as_ptr())
        }
    }

    #[doc(alias = "IRCompilerAllocCompileAndLink")]
    pub fn alloc_compile_and_link<'a>(
        // TODO: This was mut, why?
        &'a self,
        entry_point: &CStr,
        input: &IRObject<'a>,
    ) -> Result<IRObject<'a>, CompilerError> {
        let mut error = std::ptr::null_mut();

        let v = NonNull::new(unsafe {
            self.funcs.IRCompilerAllocCompileAndLink(
                self.me.as_ptr(),
                entry_point.as_ptr(),
                input.me.as_ptr(),
                &mut error,
            )
        });

        if error.is_null() {
            Ok(IRObject {
                me: v.unwrap(),
                funcs: input.funcs,
                compiler: self,
            })
        } else {
            let error = IRError::from_ptr(error, self.funcs);
            Err(CompilerError(error.code()))
        }
    }
}

struct IRError <'lib>{
    me: NonNull<bindings::IRError>,
    funcs: &'lib bindings::metal_irconverter,
}

impl<'lib> Drop for IRError <'lib>{
    #[doc(alias = "IRErrorDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRErrorDestroy(self.me.as_ptr()) };
    }
}

impl<'lib> IRError<'lib> {
    pub fn from_ptr(ptr: *mut bindings::IRError, funcs: &'lib bindings::metal_irconverter) -> Self {
        Self {
            me: NonNull::new(ptr).unwrap(),
            funcs,
        }
    }

    #[doc(alias = "IRErrorGetCode")]
    pub fn code(&self) -> IRErrorCode {
        let code = unsafe { self.funcs.IRErrorGetCode(self.me.as_ptr()) };
        match code {
            0 => IRErrorCode::NoError,
            1 => IRErrorCode::ShaderRequiresRootSignature,
            2 => IRErrorCode::UnrecognizedRootSignatureDescriptor,
            3 => IRErrorCode::UnrecognizedParameterTypeInRootSignature,
            4 => IRErrorCode::ResourceNotReferencedByRootSignature,
            5 => IRErrorCode::ShaderIncompatibleWithDualSourceBlending,
            6 => IRErrorCode::UnsupportedWaveSize,
            7 => IRErrorCode::UnsupportedInstruction,
            8 => IRErrorCode::CompilationError,
            9 => IRErrorCode::FailedToSynthesizeStageInFunction,
            10 => IRErrorCode::FailedToSynthesizeStreamOutFunction,
            11 => IRErrorCode::FailedToSynthesizeIndirectIntersectionFunction,
            12 => IRErrorCode::UnableToVerifyModule,
            13 => IRErrorCode::UnableToLinkModule,
            14 => IRErrorCode::UnrecognizedDXILHeader,
            15 => IRErrorCode::InvalidRaytracingAttribute,
            16 => IRErrorCode::Unknown,
            _ => panic!("Invalid error code"),
        }
    }

    #[doc(alias = "IRErrorGetPayload")]
    pub fn _payload(&self) -> CString {
        unsafe {
            // The documentation is inconsistent about this function.
            // The docs say `You must cast this pointer to the appropriate error payload struct for the error code``
            // but the example code just treats this as a char* like this: `printf("%s\n", (const char*)IRErrorGetPayload(pRootSigError));`
            // Let's assume the example code is correct
            let payload = self.funcs.IRErrorGetPayload(self.me.as_ptr()) as *mut i8;
            CString::from_raw(payload)
        }
    }
}
