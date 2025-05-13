#![doc = include_str!("../README.md")]
#![deny(clippy::use_self, clippy::unwrap_used, rust_2018_idioms)]
use std::{
    error,
    ffi::{c_char, CStr, OsStr},
    fmt,
    marker::PhantomData,
    mem::MaybeUninit,
    ops::Deref,
    ptr::NonNull,
    sync::Arc,
};

#[expect(
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
    clippy::use_self,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod bindings;
pub use bindings as ffi;
use thiserror::Error;

/// [`MetalIrConverter`] is used to load the `metal_irconverter` dynamic library and holds its
/// functions in an [`Arc`]. Since [`IRCompiler`] is not thread-safe, this struct provides an
/// interface to create [`IRCompiler`] instances as well as other objects provided by the library
/// ([`IRObject`], [`IRRootSignature`]), without needing an [`IRCompiler`] instance. This way, the
/// library only has to be loaded once, but each thread can have its own [`IRCompiler`] instance.
#[derive(Clone)]
pub struct MetalIrConverter {
    funcs: Arc<bindings::metal_irconverter>,
}

impl MetalIrConverter {
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

    #[doc(alias = "IRObjectCreateFromDXIL")]
    pub fn create_object_from_dxil(&self, bytecode: &[u8]) -> IRObject {
        unsafe {
            let me = NonNull::new(self.funcs.IRObjectCreateFromDXIL(
                bytecode.as_ptr(),
                bytecode.len(),
                // TODO: This creates a copy of the data.  We could also set this to None
                // with a PhantdomData lifetime on the incoming bytecode slice
                bindings::IRBytecodeOwnership::Copy,
            ))
            .expect("Failed to create IRObject from DXIL");

            IRObject {
                me,
                funcs: self.funcs.clone(),
            }
        }
    }

    #[doc(alias = "IRRootSignatureCreateFromDescriptor")]
    pub fn create_root_signature_from_descriptor(
        &self,
        desc: &ffi::IRVersionedRootSignatureDescriptor,
    ) -> Result<IRRootSignature, RootSignatureError> {
        let mut error = std::ptr::null_mut();

        let me = NonNull::new(unsafe {
            self.funcs
                .IRRootSignatureCreateFromDescriptor(desc, &mut error)
        });

        if let Some(error) = NonNull::new(error) {
            let error = unsafe { IRError::from_ptr(error, self.funcs.clone()) };
            return Err(RootSignatureError(error));
        }

        let me =
            me.expect("IRRootSignatureCreateFromDescriptor should not return NULL without error");
        Ok(IRRootSignature {
            me,
            funcs: self.funcs.clone(),
        })
    }
}

macro_rules! versioned_info {
    ($name:ident, $create:ident, $release:ident) => {
        pub struct $name {
            me: ffi::$name,
            funcs: Arc<bindings::metal_irconverter>,
        }

        impl Deref for $name {
            type Target = ffi::$name;

            fn deref(&self) -> &Self::Target {
                &self.me
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                assert!(unsafe { self.funcs.$release(&mut self.me) })
            }
        }

        impl $name {
            fn new(
                reflection: &IRShaderReflection,
                version: ffi::IRReflectionVersion,
            ) -> Option<Self> {
                let mut info = MaybeUninit::uninit();
                if unsafe {
                    reflection
                        .funcs
                        .$create(reflection.me.as_ptr(), version, info.as_mut_ptr())
                } {
                    Some(Self {
                        me: unsafe { info.assume_init() },
                        funcs: Arc::clone(&reflection.funcs),
                    })
                } else {
                    None
                }
            }
        }
    };
}

versioned_info!(
    IRVersionedCSInfo,
    IRShaderReflectionCopyComputeInfo,
    IRShaderReflectionReleaseComputeInfo
);

versioned_info!(
    IRVersionedVSInfo,
    IRShaderReflectionCopyVertexInfo,
    IRShaderReflectionReleaseVertexInfo
);

versioned_info!(
    IRVersionedFSInfo,
    IRShaderReflectionCopyFragmentInfo,
    IRShaderReflectionReleaseFragmentInfo
);

versioned_info!(
    IRVersionedGSInfo,
    IRShaderReflectionCopyGeometryInfo,
    IRShaderReflectionReleaseGeometryInfo
);

versioned_info!(
    IRVersionedHSInfo,
    IRShaderReflectionCopyHullInfo,
    IRShaderReflectionReleaseHullInfo
);

versioned_info!(
    IRVersionedDSInfo,
    IRShaderReflectionCopyDomainInfo,
    IRShaderReflectionReleaseDomainInfo
);

versioned_info!(
    IRVersionedMSInfo,
    IRShaderReflectionCopyMeshInfo,
    IRShaderReflectionReleaseMeshInfo
);

versioned_info!(
    IRVersionedASInfo,
    IRShaderReflectionCopyAmplificationInfo,
    IRShaderReflectionReleaseAmplificationInfo
);

versioned_info!(
    IRVersionedRTInfo,
    IRShaderReflectionCopyRaytracingInfo,
    IRShaderReflectionReleaseRaytracingInfo
);

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
    /// **Private** function that's not on [`MetalIrConverter`] because it is only used internally
    /// to return initialized objects.
    #[doc(alias = "IRShaderReflectionCreate")]
    fn new(funcs: Arc<bindings::metal_irconverter>) -> Self {
        let me = NonNull::new(unsafe { funcs.IRShaderReflectionCreate() })
            .expect("Failed to create IRShaderReflection");
        Self { me, funcs }
    }

    #[doc(alias = "IRShaderReflectionCopyVertexInfo")]
    pub fn vertex_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedVSInfo> {
        IRVersionedVSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyFragmentInfo")]
    pub fn fragment_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedFSInfo> {
        IRVersionedFSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyGeometryInfo")]
    pub fn geometry_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedGSInfo> {
        IRVersionedGSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyHullInfo")]
    pub fn hull_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedHSInfo> {
        IRVersionedHSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyDomainInfo")]
    pub fn domain_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedDSInfo> {
        IRVersionedDSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyMeshInfo")]
    pub fn mesh_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedMSInfo> {
        IRVersionedMSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyAmplificationInfo")]
    pub fn amplification_info(
        &self,
        version: ffi::IRReflectionVersion,
    ) -> Option<IRVersionedASInfo> {
        IRVersionedASInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyComputeInfo")]
    pub fn compute_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedCSInfo> {
        IRVersionedCSInfo::new(self, version)
    }

    #[doc(alias = "IRShaderReflectionCopyRaytracingInfo")]
    pub fn raytracing_info(&self, version: ffi::IRReflectionVersion) -> Option<IRVersionedRTInfo> {
        IRVersionedRTInfo::new(self, version)
    }
}

pub struct IRObject {
    me: NonNull<bindings::IRObject>,
    funcs: Arc<bindings::metal_irconverter>,
}

impl Drop for IRObject {
    #[doc(alias = "IRObjectDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRObjectDestroy(self.me.as_ptr()) }
    }
}

impl IRObject {
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
    pub fn metal_lib_binary(&self) -> Option<IRMetalLibBinary> {
        let binary = IRMetalLibBinary::new(self.funcs.clone());
        if unsafe {
            self.funcs.IRObjectGetMetalLibBinary(
                self.me.as_ptr(),
                self.metal_ir_shader_stage(),
                binary.me.as_ptr(),
            )
        } {
            Some(binary)
        } else {
            None
        }
    }

    #[doc(alias = "IRObjectGetReflection")]
    pub fn reflection(&self) -> Option<IRShaderReflection> {
        let reflection = IRShaderReflection::new(self.funcs.clone());
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
    /// **Private** function that's not on [`MetalIrConverter`] because it is only used internally
    /// to return initialized objects.
    #[doc(alias = "IRMetalLibBinaryCreate")]
    fn new(funcs: Arc<bindings::metal_irconverter>) -> Self {
        unsafe {
            let me = NonNull::new(funcs.IRMetalLibBinaryCreate())
                .expect("Failed to create empty IRMetalLibBinary");
            Self { me, funcs }
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

/// Captures errors returned by [`MetalIrConverter::create_root_signature_from_descriptor()`].
#[derive(Error, Debug)]
#[error("IRRootSignature creation failed: {0:?}")]
pub struct RootSignatureError(IRError);

impl IRRootSignature {
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

/// Captures errors returned by [`IRCompiler::alloc_compile_and_link()`].
#[derive(Error, Debug)]
#[error("Compilation failed: {0:?}")]
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

#[derive(Clone, Copy, Debug)]
pub struct IRInputLayoutDescriptor1<'a>(ffi::IRInputLayoutDescriptor1, PhantomData<&'a CStr>);

impl<'a> IRInputLayoutDescriptor1<'a> {
    pub fn new(
        semantic_names: &[&'a CStr],
        input_element_descs: &[ffi::IRInputElementDescriptor1],
    ) -> Self {
        const MAX_ELEMENTS: usize = 31; // Hardcoded in the struct

        assert_eq!(semantic_names.len(), input_element_descs.len());
        assert!(semantic_names.len() <= MAX_ELEMENTS);

        let mut s = ffi::IRInputLayoutDescriptor1 {
            semanticNames: [std::ptr::null(); MAX_ELEMENTS],
            inputElementDescs: [unsafe { std::mem::zeroed() }; MAX_ELEMENTS],
            numElements: semantic_names.len() as u32,
        };

        for (i, (name, element)) in semantic_names.iter().zip(input_element_descs).enumerate() {
            s.semanticNames[i] = name.as_ptr();
            s.inputElementDescs[i] = *element;
        }

        Self(s, PhantomData)
    }
}

impl<'a> From<IRInputLayoutDescriptor1<'a>> for ffi::IRVersionedInputLayoutDescriptor {
    fn from(value: IRInputLayoutDescriptor1<'a>) -> Self {
        Self {
            version: bindings::IRInputLayoutDescriptorVersion::_1,
            u_1: ffi::IRVersionedInputLayoutDescriptor_u { desc_1_0: value.0 },
        }
    }
}

impl IRCompiler {
    #[doc(alias = "IRCompilerSetValidationFlags")]
    pub fn set_validation_flags(&mut self, validation_flags: ffi::IRCompilerValidationFlags) {
        unsafe {
            self.funcs
                .IRCompilerSetValidationFlags(self.me.as_ptr(), validation_flags)
        }
    }

    #[doc(alias = "IRCompilerSetStageInGenerationMode")]
    pub fn set_stage_in_generation_mode(&mut self, stage: ffi::IRStageInCodeGenerationMode) {
        unsafe {
            self.funcs
                .IRCompilerSetStageInGenerationMode(self.me.as_ptr(), stage)
        }
    }

    #[must_use]
    #[doc(alias = "IRMetalLibSynthesizeStageInFunction")]
    pub fn synthesize_stage_in_function(
        &mut self,
        vertex_shader_reflection: &IRShaderReflection,
        layout: impl Into<ffi::IRVersionedInputLayoutDescriptor>,
        binary: &IRMetalLibBinary,
    ) -> bool {
        unsafe {
            self.funcs.IRMetalLibSynthesizeStageInFunction(
                self.me.as_ptr(),
                vertex_shader_reflection.me.as_ptr(),
                &layout.into(),
                binary.me.as_ptr(),
            )
        }
    }

    #[doc(alias = "IRCompilerSetGlobalRootSignature")]
    pub fn set_global_root_signature(&mut self, root_signature: &IRRootSignature) {
        unsafe {
            self.funcs
                .IRCompilerSetGlobalRootSignature(self.me.as_ptr(), root_signature.me.as_ptr())
        }
    }

    #[doc(alias = "IRCompilerSetLocalRootSignature")]
    pub fn set_local_root_signature(&mut self, root_signature: &IRRootSignature) {
        unsafe {
            self.funcs
                .IRCompilerSetLocalRootSignature(self.me.as_ptr(), root_signature.me.as_ptr())
        }
    }

    #[doc(alias = "IRCompilerSetHitgroupType")]
    pub fn set_hitgroup_type(&mut self, hit_group_type: ffi::IRHitGroupType) {
        unsafe {
            self.funcs
                .IRCompilerSetHitgroupType(self.me.as_ptr(), hit_group_type)
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
        ray_generation_compilation_mode: ffi::IRRayGenerationCompilationMode,
        intersection_function_compilation_mode: ffi::IRIntersectionFunctionCompilationMode,
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
                ray_generation_compilation_mode,
                intersection_function_compilation_mode,
            )
        }
    }

    #[doc(alias = "IRCompilerSetCompatibilityFlags")]
    pub fn set_compatibility_flags(&mut self, flags: ffi::IRCompatibilityFlags) {
        unsafe {
            self.funcs
                .IRCompilerSetCompatibilityFlags(self.me.as_ptr(), flags)
        }
    }

    #[doc(alias = "IRCompilerSetInputTopology")]
    pub fn set_input_topology(&mut self, input_topology: ffi::IRInputTopology) {
        unsafe {
            self.funcs
                .IRCompilerSetInputTopology(self.me.as_ptr(), input_topology)
        }
    }

    #[doc(alias = "IRCompilerEnableGeometryAndTessellationEmulation")]
    pub fn enable_geometry_and_tessellation_emulation(&mut self, enable: bool) {
        unsafe {
            self.funcs
                .IRCompilerEnableGeometryAndTessellationEmulation(self.me.as_ptr(), enable)
        }
    }

    #[doc(alias = "IRCompilerSetDualSourceBlendingConfiguration")]
    pub fn set_dual_source_blending_configuration(
        &mut self,
        configuration: ffi::IRDualSourceBlendingConfiguration,
    ) {
        unsafe {
            self.funcs
                .IRCompilerSetDualSourceBlendingConfiguration(self.me.as_ptr(), configuration)
        }
    }

    #[doc(alias = "IRCompilerSetDepthFeedbackConfiguration")]
    pub fn set_depth_feedback_configuration(
        &mut self,
        configuration: ffi::IRDepthFeedbackConfiguration,
    ) {
        unsafe {
            self.funcs
                .IRCompilerSetDepthFeedbackConfiguration(self.me.as_ptr(), configuration)
        }
    }

    #[doc(alias = "IRCompilerSetIntRTMask")]
    pub fn set_int_rt_mask(&mut self, int_rt_mask: u8) {
        unsafe {
            self.funcs
                .IRCompilerSetIntRTMask(self.me.as_ptr(), int_rt_mask)
        }
    }

    #[doc(alias = "IRMetalLibSynthesizeIndirectRayDispatchFunction")]
    pub fn synthesize_indirect_ray_dispatch_function(&mut self) -> Option<IRMetalLibBinary> {
        let binary = IRMetalLibBinary::new(self.funcs.clone());
        if unsafe {
            self.funcs.IRMetalLibSynthesizeIndirectRayDispatchFunction(
                self.me.as_ptr(),
                binary.me.as_ptr(),
            )
        } {
            Some(binary)
        } else {
            None
        }
    }

    #[doc(alias = "IRMetalLibSynthesizeIndirectIntersectionFunction")]
    pub fn synthesize_indirect_intersection_function(&mut self) -> Option<IRMetalLibBinary> {
        let binary = IRMetalLibBinary::new(self.funcs.clone());
        if unsafe {
            self.funcs.IRMetalLibSynthesizeIndirectIntersectionFunction(
                self.me.as_ptr(),
                binary.me.as_ptr(),
            )
        } {
            Some(binary)
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

    /// See <https://developer.apple.com/documentation/metal/mtlgpufamily> for a list of GPU
    /// families and the hardware that they correspond to.
    /// See <https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf> for a specific
    /// table denoting supported features per GPU family.
    #[doc(alias = "IRCompilerSetMinimumGPUFamily")]
    pub fn set_minimum_gpu_family(&mut self, family: ffi::IRGPUFamily) {
        unsafe {
            self.funcs
                .IRCompilerSetMinimumGPUFamily(self.me.as_ptr(), family)
        }
    }

    #[doc(alias = "IRCompilerIgnoreRootSignature")]
    pub fn ignore_root_signature(&mut self, ignore_embedded_root_signature: bool) {
        unsafe {
            self.funcs
                .IRCompilerIgnoreRootSignature(self.me.as_ptr(), ignore_embedded_root_signature)
        }
    }

    #[doc(alias = "IRCompilerIgnoreDebugInformation")]
    pub fn ignore_debug_information(&mut self, ignore_debug_information: bool) {
        unsafe {
            self.funcs
                .IRCompilerIgnoreDebugInformation(self.me.as_ptr(), ignore_debug_information)
        }
    }

    #[doc(alias = "IRCompilerSetMinimumDeploymentTarget")]
    pub fn set_minimum_deployment_target(
        &mut self,
        operating_system: ffi::IROperatingSystem,
        version: &CStr,
    ) {
        unsafe {
            self.funcs.IRCompilerSetMinimumDeploymentTarget(
                self.me.as_ptr(),
                operating_system,
                version.as_ptr(),
            )
        }
    }

    #[doc(alias = "IRCompilerAllocCompileAndLink")]
    pub fn alloc_compile_and_link(
        &self,
        entry_point: &CStr,
        input: &IRObject,
    ) -> Result<IRObject, CompilerError> {
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
            funcs: self.funcs.clone(),
        })
    }

    #[doc(alias = "IRCompilerAllocCombineCompileAndLink")]
    pub fn alloc_combine_compile_and_link(
        &self,
        intersection_function_entry_point: &CStr,
        intersection_function_bytecode: &IRObject,
        any_hit_function_entry_point: &CStr,
        any_hit_function_bytecode: &IRObject,
    ) -> Result<IRObject, CompilerError> {
        let mut error = std::ptr::null_mut();

        let object = NonNull::new(unsafe {
            self.funcs.IRCompilerAllocCombineCompileAndLink(
                self.me.as_ptr(),
                intersection_function_entry_point.as_ptr(),
                intersection_function_bytecode.me.as_ptr(),
                any_hit_function_entry_point.as_ptr(),
                any_hit_function_bytecode.me.as_ptr(),
                &mut error,
            )
        });

        if let Some(error) = NonNull::new(error) {
            let error = unsafe { IRError::from_ptr(error, self.funcs.clone()) };
            return Err(CompilerError(error));
        }

        let object = object
            .expect("IRCompilerAllocCombineCompileAndLink should not return NULL without error");
        Ok(IRObject {
            me: object,
            funcs: self.funcs.clone(),
        })
    }
}

pub struct IRError {
    me: NonNull<bindings::IRError>,
    funcs: Arc<bindings::metal_irconverter>,
}

// The underlying read-only error value and raw pointer are likely thread-safe, and don't reference
// anything from `IRCompiler`.  Unsafely implement this to be usable in an anyhow chain:
unsafe impl Send for IRError {}
unsafe impl Sync for IRError {}

impl fmt::Debug for IRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IRError")
            .field("code", &self.code())
            .field("payload", &self.payload())
            .finish()
    }
}

impl fmt::Display for IRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IRError {:?}: {:?}", self.code(), self.payload())
    }
}

impl error::Error for IRError {}

impl Drop for IRError {
    #[doc(alias = "IRErrorDestroy")]
    fn drop(&mut self) {
        unsafe { self.funcs.IRErrorDestroy(self.me.as_ptr()) };
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
    pub fn payload(&self) -> &CStr {
        unsafe {
            // The documentation is inconsistent about this function.
            // The docs say `You must cast this pointer to the appropriate error payload struct for the error code``
            // but the example code just treats this as a char* like this: `printf("%s\n", (const char*)IRErrorGetPayload(pRootSigError));`
            // Let's assume the example code is correct
            let payload = self.funcs.IRErrorGetPayload(self.me.as_ptr()) as *mut c_char;
            CStr::from_ptr(payload)
        }
    }
}
