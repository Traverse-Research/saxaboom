#![doc = include_str!("../README.md")]
#![deny(clippy::use_self)]
use std::{
    ffi::{c_char, CStr, CString, OsStr},
    mem::MaybeUninit,
    ptr::NonNull,
    sync::Arc,
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

pub struct IRShaderReflection {
    me: NonNull<bindings::IRShaderReflection>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRShaderReflection {
    #[doc(alias = "IRShaderReflectionDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRShaderReflectionDestroy(self.me.as_ptr()) }
    }
}

impl IRShaderReflection {
    #[doc(alias = "IRShaderReflectionCreate")]
    pub fn new(compiler: &IRCompiler) -> Self {
        let me = NonNull::new(unsafe { compiler.funcs.IRShaderReflectionCreate() })
            .expect("Failed to create IRShaderReflection");
        Self {
            me,
            funcs: compiler.funcs.clone(),
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
#[error("Failed to get MetalLib bytecode from IRObject: {0:?}, {1:?}")]
pub struct MetalLibNoBytecodeFoundError(ffi::IRObjectType, ffi::IRShaderStage);

pub struct IRObject<'a> {
    me: NonNull<bindings::IRObject>,
    funcs: Arc<bindings::metal_irconverter>,
    compiler: &'a IRCompiler,
}

impl<'a> Drop for IRObject<'a> {
    #[doc(alias = "IRObjectDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRObjectDestroy(self.me.as_ptr()) }
    }
}

impl<'a> IRObject<'a> {
    #[doc(alias = "IRObjectCreateFromDXIL")]
    pub fn create_from_dxil(compiler: &'a IRCompiler, bytecode: &[u8]) -> IRObject<'a> {
        unsafe {
            let me = NonNull::new(compiler.funcs.IRObjectCreateFromDXIL(
                bytecode.as_ptr(),
                bytecode.len(),
                bindings::IRBytecodeOwnership::Copy,
            ))
            .expect("Failed to create IRObject from DXIL");

            Self {
                me,
                funcs: compiler.funcs.clone(),
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
    ) -> Result<IRMetalLibBinary, MetalLibNoBytecodeFoundError> {
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
    pub fn reflection(&self, shader_stage: ffi::IRShaderStage) -> Option<IRShaderReflection> {
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

pub struct IRMetalLibBinary {
    me: NonNull<bindings::IRMetalLibBinary>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRMetalLibBinary {
    #[doc(alias = "IRMetalLibBinaryDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRMetalLibBinaryDestroy(self.me.as_ptr()) }
    }
}

impl IRMetalLibBinary {
    #[doc(alias = "IRMetalLibBinaryCreate")]
    pub fn new(compiler: &IRCompiler) -> Self {
        unsafe {
            let me = NonNull::new(compiler.funcs.IRMetalLibBinaryCreate())
                .expect("Failed to create empty IRMetalLibBinary");
            Self {
                me,
                funcs: compiler.funcs.clone(),
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
        assert_eq!(written, size_in_bytes);
        unsafe { bytes.set_len(written) }
        bytes
    }
}

pub struct IRRootSignature {
    me: NonNull<bindings::IRRootSignature>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRRootSignature {
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

impl IRRootSignature {
    #[doc(alias = "IRRootSignatureCreateFromDescriptor")]
    pub fn create_from_descriptor(
        compiler: &IRCompiler,
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
            let error = IRError::from_ptr(error, compiler.funcs.clone());
            Err(RootSignatureError::CreateError(error.code()))
        } else {
            match me {
                Some(me) => Ok(Self {
                    me,
                    funcs: compiler.funcs.clone(),
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

    #[doc(alias = "IRCompilerCreate")]
    pub fn create_compiler(&self) -> IRCompiler {
        let compiler = NonNull::new(unsafe { self.funcs.IRCompilerCreate() })
            .expect("Failed to create IRCompiler");
        IRCompiler {
            me: compiler,
            funcs: self.funcs.clone(),
        }
    }
}

#[derive(Error, Debug)]
#[error("Failed to compile IRObject: ({0:?})")]
pub struct CompilerError(ffi::IRErrorCode);

/// This object is not thread-safe, refer to [the Metal shader converter documentation], the "Multithreading considerations" chapter.
///
/// [the Metal shader converter documentation]: https://developer.apple.com/metal/shader-converter/
pub struct IRCompiler {
    me: NonNull<bindings::IRCompiler>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRCompiler {
    #[doc(alias = "IRCompilerDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRCompilerDestroy(self.me.as_ptr()) }
    }
}

impl IRCompiler {
    #[doc(alias = "IRCompilerSetGlobalRootSignature")]
    pub fn set_global_root_signature(&mut self, root_signature: &IRRootSignature) {
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
        closest_hit_intrinsics_mask: u64,
        miss_intrinsics_mask: u64,
        any_hit_intrinsics_mask: u64,
        callable_args: u64,
        max_recursive_depth: i32,
    ) {
        unsafe {
            self.funcs.IRCompilerSetRayTracingPipelineArguments(
                self.me.as_ptr(),
                max_attribute_size_in_bytes,
                raytracing_pipeline_flags,
                closest_hit_mask,
                miss_mask,
                any_hit_mask,
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
    pub fn synthesize_indirect_intersection_function(&mut self) -> Option<IRMetalLibBinary> {
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
    pub fn synthesize_indirect_ray_dispatch_function(&mut self) -> Option<IRMetalLibBinary> {
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
                funcs: input.funcs.clone(),
                compiler: self,
            })
        } else {
            let error = IRError::from_ptr(error, self.funcs.clone());
            Err(CompilerError(error.code()))
        }
    }
}

struct IRError {
    me: NonNull<bindings::IRError>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRError {
    #[doc(alias = "IRErrorDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRErrorDestroy(self.me.as_ptr()) };
    }
}

impl IRError {
    pub fn from_ptr(ptr: *mut bindings::IRError, funcs: Arc<bindings::metal_irconverter>) -> Self {
        Self {
            me: NonNull::new(ptr).unwrap(),
            funcs,
        }
    }

    #[doc(alias = "IRErrorGetCode")]
    pub fn code(&self) -> IRErrorCode {
        let code = unsafe { self.funcs.IRErrorGetCode(self.me.as_ptr()) };
        match code {
            x if x == IRErrorCode::NoError as u32 => IRErrorCode::NoError,
            x if x == IRErrorCode::ShaderRequiresRootSignature as u32 => {
                IRErrorCode::ShaderRequiresRootSignature
            }
            x if x == IRErrorCode::UnrecognizedRootSignatureDescriptor as u32 => {
                IRErrorCode::UnrecognizedRootSignatureDescriptor
            }
            x if x == IRErrorCode::UnrecognizedParameterTypeInRootSignature as u32 => {
                IRErrorCode::UnrecognizedParameterTypeInRootSignature
            }
            x if x == IRErrorCode::ResourceNotReferencedByRootSignature as u32 => {
                IRErrorCode::ResourceNotReferencedByRootSignature
            }
            x if x == IRErrorCode::ShaderIncompatibleWithDualSourceBlending as u32 => {
                IRErrorCode::ShaderIncompatibleWithDualSourceBlending
            }
            x if x == IRErrorCode::UnsupportedWaveSize as u32 => IRErrorCode::UnsupportedWaveSize,
            x if x == IRErrorCode::UnsupportedInstruction as u32 => {
                IRErrorCode::UnsupportedInstruction
            }
            x if x == IRErrorCode::CompilationError as u32 => IRErrorCode::CompilationError,
            x if x == IRErrorCode::FailedToSynthesizeStageInFunction as u32 => {
                IRErrorCode::FailedToSynthesizeStageInFunction
            }
            x if x == IRErrorCode::FailedToSynthesizeStreamOutFunction as u32 => {
                IRErrorCode::FailedToSynthesizeStreamOutFunction
            }
            x if x == IRErrorCode::FailedToSynthesizeIndirectIntersectionFunction as u32 => {
                IRErrorCode::FailedToSynthesizeIndirectIntersectionFunction
            }
            x if x == IRErrorCode::UnableToVerifyModule as u32 => IRErrorCode::UnableToVerifyModule,
            x if x == IRErrorCode::UnableToLinkModule as u32 => IRErrorCode::UnableToLinkModule,
            x if x == IRErrorCode::UnrecognizedDXILHeader as u32 => {
                IRErrorCode::UnrecognizedDXILHeader
            }
            x if x == IRErrorCode::InvalidRaytracingAttribute as u32 => {
                IRErrorCode::InvalidRaytracingAttribute
            }
            x if x == IRErrorCode::Unknown as u32 => IRErrorCode::Unknown,
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
            let payload = self.funcs.IRErrorGetPayload(self.me.as_ptr()) as *mut c_char;
            CString::from_raw(payload)
        }
    }
}
