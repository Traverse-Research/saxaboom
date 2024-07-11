#![doc = include_str!("../README.md")]
#![deny(clippy::use_self, clippy::unwrap_used)]
use std::{
    ffi::{c_char, CStr, CString, OsStr},
    fmt,
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
#[error("Failed to get MetalLib bytecode for stage `{stage:?}` from IRObject `{object_type:?}`")]
pub struct MetalLibNoBytecodeFoundError {
    object_type: ffi::IRObjectType,
    stage: ffi::IRShaderStage,
}

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
    pub fn create_from_dxil(compiler: &'a IRCompiler, bytecode: &[u8]) -> Self {
        unsafe {
            let me = NonNull::new(compiler.funcs.IRObjectCreateFromDXIL(
                bytecode.as_ptr(),
                bytecode.len(),
                // TODO: This creates a copy of the data.  We could also set this to None
                // with a PhantdomData lifetime on the incoming bytecode slice
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
    pub fn r#type(&self) -> ffi::IRObjectType {
        unsafe { self.funcs.IRObjectGetType(self.me.as_ptr()) }
    }

    #[doc(alias = "IRObjectGetMetalIRShaderStage")]
    pub fn metal_ir_shader_stage(&self) -> ffi::IRShaderStage {
        unsafe { self.funcs.IRObjectGetMetalIRShaderStage(self.me.as_ptr()) }
    }

    #[doc(alias = "IRObjectGetMetalLibBinary")]
    pub fn metal_lib_binary(&self) -> Result<IRMetalLibBinary, MetalLibNoBytecodeFoundError> {
        let mtl_lib = IRMetalLibBinary::new(self.compiler);
        if unsafe {
            self.funcs.IRObjectGetMetalLibBinary(
                self.me.as_ptr(),
                self.metal_ir_shader_stage(),
                mtl_lib.me.as_ptr(),
            )
        } {
            Ok(mtl_lib)
        } else {
            Err(MetalLibNoBytecodeFoundError {
                object_type: self.r#type(),
                stage: self.metal_ir_shader_stage(),
            })
        }
    }

    #[doc(alias = "IRObjectGetReflection")]
    pub fn reflection(&self) -> Option<IRShaderReflection> {
        let reflection = IRShaderReflection::new(self.compiler);
        if unsafe {
            self.funcs.IRObjectGetReflection(
                self.me.as_ptr(),
                self.metal_ir_shader_stage(),
                reflection.me.as_ptr(),
            )
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
#[error("IRRootSignature creation failed: {0:?}")]
pub struct RootSignatureError(IRError);

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

        if let Some(error) = NonNull::new(error) {
            let error = unsafe { IRError::from_ptr(error, compiler.funcs.clone()) };
            return Err(RootSignatureError(error));
        }

        let me =
            me.expect("IRRootSignatureCreateFromDescriptor should not return NULL without error");
        Ok(Self {
            me,
            funcs: compiler.funcs.clone(),
        })
    }

    #[doc(alias(
        "IRRootSignatureGetResourceCount",
        "IRRootSignatureGetResourceLocations"
    ))]
    pub fn resource_locations(&self) -> Vec<ffi::IRResourceLocation> {
        let n_resources = unsafe { self.funcs.IRRootSignatureGetResourceCount(self.me.as_ptr()) };
        let mut resource_locations = Vec::with_capacity(n_resources);
        unsafe {
            self.funcs.IRRootSignatureGetResourceLocations(
                self.me.as_ptr(),
                resource_locations.as_mut_ptr(),
            );
            resource_locations.set_len(n_resources)
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
#[error("IRObject compilation failed: {0:?}")]
pub struct CompilerError(IRError);

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
                closest_hit_intrinsics_mask,
                miss_intrinsics_mask,
                any_hit_intrinsics_mask,
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
        &'a self,
        entry_point: &CStr,
        input: &IRObject<'a>,
    ) -> Result<IRObject<'a>, CompilerError> {
        let mut error = std::ptr::null_mut();

        let object = NonNull::new(unsafe {
            self.funcs.IRCompilerAllocCompileAndLink(
                self.me.as_ptr(),
                entry_point.as_ptr(),
                input.me.as_ptr(),
                &mut error,
            )
        });

        if let Some(error) = NonNull::new(error) {
            let error = unsafe { IRError::from_ptr(error, self.funcs.clone()) };
            return Err(CompilerError(error));
        }

        let object =
            object.expect("IRCompilerAllocCompileAndLink should not return NULL without error");
        Ok(IRObject {
            me: object,
            funcs: input.funcs.clone(),
            compiler: self,
        })
    }
}

pub struct IRError {
    me: NonNull<bindings::IRError>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRError {
    #[doc(alias = "IRErrorDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRErrorDestroy(self.me.as_ptr()) };
    }
}

impl fmt::Debug for IRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IRError")
            .field("code", &self.code())
            .field("payload", &self.payload())
            .finish()
    }
}

impl IRError {
    unsafe fn from_ptr(
        me: NonNull<bindings::IRError>,
        funcs: Arc<bindings::metal_irconverter>,
    ) -> Self {
        Self { me, funcs }
    }

    #[doc(alias = "IRErrorGetCode")]
    pub fn code(&self) -> ffi::IRErrorCode {
        let code = unsafe { self.funcs.IRErrorGetCode(self.me.as_ptr()) };
        use ffi::IRErrorCode::*;
        match code {
            x if x == NoError as u32 => NoError,
            x if x == ShaderRequiresRootSignature as u32 => ShaderRequiresRootSignature,
            x if x == UnrecognizedRootSignatureDescriptor as u32 => {
                UnrecognizedRootSignatureDescriptor
            }
            x if x == UnrecognizedParameterTypeInRootSignature as u32 => {
                UnrecognizedParameterTypeInRootSignature
            }
            x if x == ResourceNotReferencedByRootSignature as u32 => {
                ResourceNotReferencedByRootSignature
            }
            x if x == ShaderIncompatibleWithDualSourceBlending as u32 => {
                ShaderIncompatibleWithDualSourceBlending
            }
            x if x == UnsupportedWaveSize as u32 => UnsupportedWaveSize,
            x if x == UnsupportedInstruction as u32 => UnsupportedInstruction,
            x if x == CompilationError as u32 => CompilationError,
            x if x == FailedToSynthesizeStageInFunction as u32 => FailedToSynthesizeStageInFunction,
            x if x == FailedToSynthesizeStreamOutFunction as u32 => {
                FailedToSynthesizeStreamOutFunction
            }
            x if x == FailedToSynthesizeIndirectIntersectionFunction as u32 => {
                FailedToSynthesizeIndirectIntersectionFunction
            }
            x if x == UnableToVerifyModule as u32 => UnableToVerifyModule,
            x if x == UnableToLinkModule as u32 => UnableToLinkModule,
            x if x == UnrecognizedDXILHeader as u32 => UnrecognizedDXILHeader,
            x if x == InvalidRaytracingAttribute as u32 => InvalidRaytracingAttribute,
            x if x == Unknown as u32 => Unknown,
            _ => panic!("Invalid error code"),
        }
    }

    #[doc(alias = "IRErrorGetPayload")]
    pub fn payload(&self) -> CString {
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
