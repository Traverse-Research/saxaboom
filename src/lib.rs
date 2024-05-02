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
    clippy::enum_variant_names
)]
mod bindings;

use bindings::IRError;
pub use bindings::{
    IRCSInfo_1_0, IRComparisonFunction, IRDescriptorRangeType, IRFilter, IRHitGroupType,
    IRObjectType, IRRaytracingPipelineFlags, IRReflectionVersion, IRResourceLocation,
    IRResourceType, IRRootConstants, IRRootParameter1,
    IRRootParameter1__bindgen_ty_1 as IRRootParameter1_u, IRRootParameterType,
    IRRootSignatureDescriptor1, IRRootSignatureFlags, IRRootSignatureVersion, IRShaderStage,
    IRShaderVisibility, IRStaticBorderColor, IRStaticSamplerDescriptor, IRTextureAddressMode,
    IRVersionedCSInfo, IRVersionedRootSignatureDescriptor,
    IRVersionedRootSignatureDescriptor__bindgen_ty_1 as IRVersionedRootSignatureDescriptor_u,
};

#[repr(u32)]
pub enum IRBytecodeOwnership {
    None = 0,
    Copy = 1,
}

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
        version: IRReflectionVersion,
    ) -> Result<IRVersionedCSInfo, Box<dyn std::error::Error>> {
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
                bindings::IRBytecodeOwnership::IRBytecodeOwnershipNone,
            );

            Ok(Self {
                funcs: compiler.funcs.clone(),
                me,
            })
        }
    }

    pub fn gather_raytracing_intrinsics(&self, entry_point: &CStr) -> u64 {
        unsafe {
            self.funcs
                .IRObjectGatherRaytracingIntrinsics(self.me, entry_point.as_ptr().cast())
        }
    }

    pub fn get_type(&self) -> IRObjectType {
        unsafe { self.funcs.IRObjectGetType(self.me.cast_const()) }
    }

    pub fn get_metal_ir_shader_stage(&self) -> IRShaderStage {
        unsafe { self.funcs.IRObjectGetMetalIRShaderStage(self.me) }
    }

    pub fn get_metal_lib_binary(
        &self,
        shader_stage: IRShaderStage,
        dest_lib: &mut IRMetalLibBinary,
    ) -> bool {
        unsafe {
            self.funcs
                .IRObjectGetMetalLibBinary(self.me, shader_stage, dest_lib.me)
        }
    }

    pub fn get_reflection(
        &self,
        shader_stage: IRShaderStage,
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
                funcs: compiler.funcs.clone(),
                me,
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

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRDescriptorRangeFlags {
    IRDescriptorRangeFlagNone = 0,
    IRDescriptorRangeFlagDescriptorsVolatile = 0x1,
    IRDescriptorRangeFlagDataVolatile = 0x2,
    IRDescriptorRangeFlagDataStaticWhileSetAtExecute = 0x4,
    IRDescriptorRangeFlagDataStatic = 0x8,
    IRDescriptorRangeFlagDescriptorsStaticKeepingBufferBoundsChecks = 0x10000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRDescriptorRange1 {
    pub range_type: IRDescriptorRangeType,
    pub num_descriptors: u32,
    pub base_shader_register: u32,
    pub register_space: u32,
    pub flags: IRDescriptorRangeFlags,
    pub offset_in_descriptors_from_table_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRRootDescriptorTable1 {
    pub num_descriptor_ranges: u32,
    pub p_descriptor_ranges: *const IRDescriptorRange1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRRootDescriptorFlags {
    IRRootDescriptorFlagNone = 0,
    IRRootDescriptorFlagDataVolatile = 0x2,
    IRRootDescriptorFlagDataStaticWhileSetAtExecute = 0x4,
    IRRootDescriptorFlagDataStatic = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRRootDescriptor1 {
    pub shader_register: u32,
    pub register_space: u32,
    pub flags: IRRootDescriptorFlags,
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
        desc: &IRVersionedRootSignatureDescriptor,
    ) -> Result<IRRootSignature, Box<dyn std::error::Error>> {
        unsafe {
            let mut error: *mut IRError = std::ptr::null_mut::<IRError>();

            let me = compiler
                .funcs
                .IRRootSignatureCreateFromDescriptor(desc, &mut error);

            Ok(Self {
                funcs: compiler.funcs.clone(),
                me,
            })
        }
    }

    pub fn get_resource_locations(&self) -> Vec<IRResourceLocation> {
        unsafe {
            let n_resources = self
                .funcs
                .IRRootSignatureGetResourceCount(self.me as *const _);
            let empty_location = IRResourceLocation {
                resourceType: IRResourceType::IRResourceTypeInvalid,
                space: 0,
                slot: 0,
                topLevelOffset: 0,
                sizeBytes: 0,
                resourceName: std::ptr::null(),
            };
            let mut resource_locations = vec![empty_location; n_resources];
            self.funcs.IRRootSignatureGetResourceLocations(
                self.me as *const _,
                resource_locations.as_mut_ptr(),
            );
            resource_locations
        }
    }
}

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
            funcs: self.funcs.clone(),
            me: compiler,
        }
    }
}

/// This object is not thread-safe, refer to [the Metal shader converter documentation], the "Multithreading considerations" chapter.
///
/// [the Metal shader converter documentation]: https://developer.apple.com/metal/shader-converter/
pub struct IRCompiler {
    funcs: Arc<bindings::metal_irconverter>,
    me: *mut bindings::IRCompiler,
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

    #[allow(clippy::too_many_arguments)]
    pub fn set_ray_tracing_pipeline_arguments(
        &mut self,
        max_attribute_size_in_bytes: u32,
        raytracing_pipeline_flags: IRRaytracingPipelineFlags,
        chs: u64,
        miss: u64,
        any_hit: u64,
        callable_args: u64,
        max_recursive_depth: i32,
    ) {
        unsafe {
            self.funcs.IRCompilerSetRayTracingPipelineArguments(
                self.me,
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

    pub fn set_hitgroup_type(&mut self, hit_group_type: IRHitGroupType) {
        unsafe {
            self.funcs
                .IRCompilerSetHitgroupType(self.me, hit_group_type)
        }
    }

    pub fn synthesize_indirect_intersection_function(
        &mut self,
        target_metallib: &mut IRMetalLibBinary,
    ) -> bool {
        unsafe {
            self.funcs
                .IRMetalLibSynthesizeIndirectIntersectionFunction(self.me, target_metallib.me)
        }
    }

    pub fn synthesize_indirect_ray_dispatch_function(
        &mut self,
        target_metallib: &mut IRMetalLibBinary,
    ) -> bool {
        unsafe {
            self.funcs
                .IRMetalLibSynthesizeIndirectRayDispatchFunction(self.me, target_metallib.me)
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
        let mut error: *mut IRError = std::ptr::null_mut::<IRError>();

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
                funcs: input.funcs.clone(),
                me: v,
            })
        } else {
            panic!("{:?}", error);
        }
    }
}
