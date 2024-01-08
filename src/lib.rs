#![doc = include_str!("../README.md")]

struct IRCompilerOpaque;
struct IRObjectOpaque;
struct IRRootSignatureOpaque;
struct IRMetalLibBinaryOpaque;
struct IRShaderReflectionOpaque;
struct IRErrorOpaque;

use std::mem::MaybeUninit;

#[repr(i32)]
pub enum IRReflectionVersion {
    IRReflectionVersion_1_0 = 1,
}

#[repr(i32)]
pub enum IRRaytracingPipelineFlags {
    IRRaytracingPipelineFlagNone = 0,
    IRRaytracingPipelineFlagSkipTriangles = 0x100,
    IRRaytracingPipelineFlagSkipProceduralPrimitives = 0x200,
}

#[repr(i32)]
pub enum IRHitGroupType {
    IRHitGroupTypeTriangles = 0,
    IRHitGroupTypeProceduralPrimitive = 1,
}

#[derive(Debug)]
struct IRCompilerFn<'lib> {
    alloc_compile_and_link: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut IRCompilerOpaque,
            *const u8,
            *const IRObjectOpaque,
            *mut *mut IRErrorOpaque,
        ) -> *mut IRObjectOpaque,
    >,
    create: libloading::Symbol<'lib, unsafe extern "C" fn() -> *mut IRCompilerOpaque>,
    destroy: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRCompilerOpaque) -> ()>,
    enable_geometry_and_tessellation_emulation:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRCompilerOpaque, bool) -> u32>,
    set_compatibility_flags:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRCompilerOpaque, u32) -> ()>,
    set_global_root_signature: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(*mut IRCompilerOpaque, *const IRRootSignatureOpaque),
    >,
    set_hitgroup_type:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRCompilerOpaque, IRHitGroupType) -> ()>,
    set_ray_tracing_pipeline_arguments: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut IRCompilerOpaque,
            u32,
            IRRaytracingPipelineFlags,
            u64,
            u64,
            u64,
            u64,
            ::std::os::raw::c_int,
        ) -> (),
    >,
    synthesize_indirect_intersection_function: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(*mut IRCompilerOpaque, *mut IRMetalLibBinaryOpaque) -> (),
    >,
    // todo
    set_depth_feedback_configuration: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_dual_source_blending_configuration: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_entry_point_name: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_has_geometry_shader: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_hitgroup_arguments: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_input_topology: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_local_root_signature: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_minimum_deployment_target: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_minimum_gpu_family: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_shared_rt_arguments: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_stage_in_generation_mode: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_stream_out_enabled: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_strip_cut_index: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_tessellation_enabled: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_tessellator_output_primitive: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_validation_flags: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_vertex_render_target_index_id: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_vertex_viewport_index_id: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    set_int_rt_mask: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    ignore_root_signature: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
}

#[repr(u32)]
#[derive(Debug)]
pub enum IRObjectType {
    IRObjectTypeDXILBytecode,
    IRObjectTypeMetalIRObject,
}

#[derive(Copy, Clone, Debug)]
pub struct IRCSInfo_1_0 {
    pub tg_size: [u32; 3],
}

pub union IRVersionedCSInfo_u {
    pub info_1_0: IRCSInfo_1_0,
}

pub struct IRVersionedCSInfo {
    pub version: IRReflectionVersion,
    pub u: IRVersionedCSInfo_u,
}

#[repr(u32)]
#[derive(Debug)]
pub enum IRShaderStage {
    IRShaderStageInvalid,
    IRShaderStageVertex,
    IRShaderStageFragment,
    IRShaderStageHull,
    IRShaderStageDomain,
    IRShaderStageMesh,
    IRShaderStageAmplification,
    IRShaderStageGeometry,
    IRShaderStageCompute,
    IRShaderStageClosestHit,
    IRShaderStageIntersection,
    IRShaderStageAnyHit,
    IRShaderStageMiss,
    IRShaderStageRayGeneration,
    IRShaderStageCallable,
    IRShaderStageStreamOut,
    IRShaderStageStageIn,
}

#[repr(u32)]
pub enum IRBytecodeOwnership {
    None = 0,
    Copy = 1,
}

struct IRReflectionFn<'lib> {
    create: libloading::Symbol<'lib, unsafe extern "C" fn() -> *mut IRShaderReflectionOpaque>,
    destroy: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    get_entry_point_function_name:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> *const u8>,
    needs_function_constants:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> bool>,
    get_function_constant_count:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> usize>,

    copy_function_constants:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_function_constants:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    copy_compute_info: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut IRShaderReflectionOpaque,
            version: IRReflectionVersion,
            csinfo: *mut IRVersionedCSInfo,
        ) -> bool,
    >,
    copy_vertex_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    copy_fragment_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    copy_geometry_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    copy_hull_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    copy_domain_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_compute_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_vertex_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_fragment_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_geometry_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_hull_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    release_domain_info:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    get_resource_count:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
    get_resource_locations:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRShaderReflectionOpaque) -> ()>,
}

pub struct IRShaderReflection<'lib> {
    me: *mut IRShaderReflectionOpaque,
    funcs: IRReflectionFn<'lib>,
}

impl<'lib> Drop for IRShaderReflection<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRShaderReflection<'lib> {
    pub fn new(
        lib: &'lib libloading::Library,
    ) -> Result<IRShaderReflection<'lib>, Box<dyn std::error::Error>> {
        unsafe {
            let funcs = IRReflectionFn {
                create: lib.get(b"IRShaderReflectionCreate")?,
                destroy: lib.get(b"IRShaderReflectionDestroy")?,
                get_entry_point_function_name: lib
                    .get(b"IRShaderReflectionGetEntryPointFunctionName")?,
                needs_function_constants: lib.get(b"IRShaderReflectionNeedsFunctionConstants")?,
                get_function_constant_count: lib
                    .get(b"IRShaderReflectionGetFunctionConstantCount")?,

                copy_function_constants: lib.get(b"IRShaderReflectionCopyFunctionConstants")?,
                release_function_constants: lib
                    .get(b"IRShaderReflectionReleaseFunctionConstants")?,
                copy_compute_info: lib.get(b"IRShaderReflectionCopyComputeInfo")?,
                copy_vertex_info: lib.get(b"IRShaderReflectionCopyVertexInfo")?,
                copy_fragment_info: lib.get(b"IRShaderReflectionCopyFragmentInfo")?,
                copy_geometry_info: lib.get(b"IRShaderReflectionCopyGeometryInfo")?,
                copy_hull_info: lib.get(b"IRShaderReflectionCopyHullInfo")?,
                copy_domain_info: lib.get(b"IRShaderReflectionCopyDomainInfo")?,
                release_compute_info: lib.get(b"IRShaderReflectionReleaseComputeInfo")?,
                release_vertex_info: lib.get(b"IRShaderReflectionReleaseVertexInfo")?,
                release_fragment_info: lib.get(b"IRShaderReflectionReleaseFragmentInfo")?,
                release_geometry_info: lib.get(b"IRShaderReflectionReleaseGeometryInfo")?,
                release_hull_info: lib.get(b"IRShaderReflectionReleaseHullInfo")?,
                release_domain_info: lib.get(b"IRShaderReflectionReleaseDomainInfo")?,
                get_resource_count: lib.get(b"IRShaderReflectionGetResourceCount")?,
                get_resource_locations: lib.get(b"IRShaderReflectionGetResourceLocations")?,
            };

            let me = (funcs.create)();
            Ok(Self { funcs, me })
        }
    }

    pub fn get_compute_info(
        &self,
        version: IRReflectionVersion,
    ) -> Result<IRVersionedCSInfo, Box<dyn std::error::Error>> {
        let mut info = unsafe { MaybeUninit::uninit() };
        if unsafe { (self.funcs.copy_compute_info)(self.me, version, info.as_mut_ptr()) } {
            Ok(unsafe { info.assume_init() })
        } else {
            panic!("Test me");
        }
    }
}

#[derive(Debug, Clone)]
struct IRObjectFn<'lib> {
    create_from_dxil: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(*const u8, usize, IRBytecodeOwnership) -> *mut IRObjectOpaque,
    >,
    destroy: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRObjectOpaque) -> ()>,
    get_metal_ir_shader_stage:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRObjectOpaque) -> IRShaderStage>,
    get_metal_lib_binary: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut IRObjectOpaque,
            IRShaderStage,
            *mut IRMetalLibBinaryOpaque,
        ) -> bool,
    >,
    get_reflection: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut IRObjectOpaque,
            IRShaderStage,
            *mut IRShaderReflectionOpaque,
        ) -> bool,
    >,
    get_type: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRObjectOpaque) -> IRObjectType>,
    serialize: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    gather_raytracing_intrinsics: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            input: *mut IRObjectOpaque,
            entryPoint: *const ::std::os::raw::c_char,
        ) -> u64,
    >,
}

pub struct IRObject<'lib> {
    me: *mut IRObjectOpaque,
    funcs: IRObjectFn<'lib>,
}

impl<'lib> Drop for IRObject<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRObject<'lib> {
    pub fn create_from_dxil(
        lib: &'lib libloading::Library,
        bytecode: &[u8],
    ) -> Result<IRObject<'lib>, Box<dyn std::error::Error>> {
        unsafe {
            let funcs = IRObjectFn {
                create_from_dxil: lib.get(b"IRObjectCreateFromDXIL")?,
                destroy: lib.get(b"IRObjectDestroy")?,
                gather_raytracing_intrinsics: lib.get(b"IRObjectGatherRaytracingIntrinsics")?,
                get_metal_ir_shader_stage: lib.get(b"IRObjectGetMetalIRShaderStage")?,
                get_metal_lib_binary: lib.get(b"IRObjectGetMetalLibBinary")?,
                get_reflection: lib.get(b"IRObjectGetReflection")?,
                get_type: lib.get(b"IRObjectGetType")?,
                serialize: lib.get(b"IRObjectSerialize")?,
            };

            let me = (funcs.create_from_dxil)(
                bytecode.as_ptr(),
                bytecode.len(),
                IRBytecodeOwnership::None,
            );

            Ok(Self { funcs, me })
        }
    }

    pub fn gather_raytracing_intrinsics(&self, entry_point: &str) -> u64 {
        unsafe {
            (self.funcs.gather_raytracing_intrinsics)(self.me, entry_point.as_ptr() as *const i8)
        }
    }

    pub fn get_type(&self) -> IRObjectType {
        unsafe { (self.funcs.get_type)(self.me) }
    }

    pub fn get_metal_ir_shader_stage(&self) -> IRShaderStage {
        unsafe { (self.funcs.get_metal_ir_shader_stage)(self.me) }
    }

    pub fn get_metal_lib_binary(
        &self,
        shader_stage: IRShaderStage,
        dest_lib: &mut IRMetalLibBinary,
    ) -> bool {
        unsafe { (self.funcs.get_metal_lib_binary)(self.me, shader_stage, dest_lib.me) }
    }

    pub fn get_reflection(
        &self,
        shader_stage: IRShaderStage,
        reflection: &mut IRShaderReflection,
    ) -> bool {
        unsafe { (self.funcs.get_reflection)(self.me, shader_stage, reflection.me) }
    }
}

#[derive(Debug, Clone)]
struct IRMetalLibBinaryFn<'lib> {
    create: libloading::Symbol<'lib, unsafe extern "C" fn() -> *mut IRMetalLibBinaryOpaque>,
    destroy: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRMetalLibBinaryOpaque)>,

    get_bytecode: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(*mut IRMetalLibBinaryOpaque, *mut u8) -> usize,
    >,
    get_bytecode_size:
        libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRMetalLibBinaryOpaque) -> usize>,

    synthesize_stage_in_function: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
}

pub struct IRMetalLibBinary<'lib> {
    me: *mut IRMetalLibBinaryOpaque,
    funcs: IRMetalLibBinaryFn<'lib>,
}

impl<'lib> Drop for IRMetalLibBinary<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRMetalLibBinary<'lib> {
    pub fn new(
        lib: &'lib libloading::Library,
    ) -> Result<IRMetalLibBinary<'lib>, Box<dyn std::error::Error>> {
        unsafe {
            let funcs = IRMetalLibBinaryFn {
                create: lib.get(b"IRMetalLibBinaryCreate")?,
                destroy: lib.get(b"IRMetalLibBinaryDestroy")?,
                get_bytecode: lib.get(b"IRMetalLibGetBytecode")?,
                get_bytecode_size: lib.get(b"IRMetalLibGetBytecodeSize")?,
                synthesize_stage_in_function: lib.get(b"IRMetalLibSynthesizeStageInFunction")?,
            };

            let me = (funcs.create)();
            Ok(Self { funcs, me })
        }
    }

    pub fn get_byte_code(&self) -> Vec<u8> {
        let size_in_bytes = unsafe { (self.funcs.get_bytecode_size)(self.me) };
        let mut bytes = vec![0u8; size_in_bytes];
        let written = unsafe { (self.funcs.get_bytecode)(self.me, bytes.as_mut_ptr()) };
        bytes
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRDescriptorRangeType {
    IRDescriptorRangeTypeSRV = 0,
    IRDescriptorRangeTypeUAV = 1,
    IRDescriptorRangeTypeCBV = 2,
    IRDescriptorRangeTypeSampler = 3,
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

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRRootParameterType {
    IRRootParameterTypeDescriptorTable = 0,
    IRRootParameterType32BitConstants = 1,
    IRRootParameterTypeCBV = 2,
    IRRootParameterTypeSRV = 3,
    IRRootParameterTypeUAV = 4,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRShaderVisibility {
    IRShaderVisibilityAll = 0,
    IRShaderVisibilityVertex = 1,
    IRShaderVisibilityHull = 2,
    IRShaderVisibilityDomain = 3,
    IRShaderVisibilityGeometry = 4,
    IRShaderVisibilityPixel = 5,
    IRShaderVisibilityAmplification = 6,
    IRShaderVisibilityMesh = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRRootConstants {
    pub shader_register: u32,
    pub register_space: u32,
    pub num32_bit_values: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IRRootParameter1_u {
    pub descriptor_table: IRRootDescriptorTable1,
    pub constants: IRRootConstants,
    pub descriptor: IRRootDescriptor1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRRootParameter1 {
    pub parameter_type: IRRootParameterType,
    pub u: IRRootParameter1_u,
    pub shader_visibility: IRShaderVisibility,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRFilter {
    IRFilterMinMagMipPoint = 0,
    IRFilterMinMagPointMipLinear = 0x1,
    IRFilterMinPointMagLinearMipPoint = 0x4,
    IRFilterMinPointMagMipLinear = 0x5,
    IRFilterMinLinearMagMipPoint = 0x10,
    IRFilterMinLinearMagPointMipLinear = 0x11,
    IRFilterMinMagLinearMipPoint = 0x14,
    IRFilterMinMagMipLinear = 0x15,
    IRFilterAnisotropic = 0x55,
    IRFilterComparisonMinMagMipPoint = 0x80,
    IRFilterComparisonMinMagPointMipLinear = 0x81,
    IRFilterComparisonMinPointMagLinearMipPoint = 0x84,
    IRFilterComparisonMinPointMagMipLinear = 0x85,
    IRFilterComparisonMinLinearMagMipPoint = 0x90,
    IRFilterComparisonMinLinearMagPointMipLinear = 0x91,
    IRFilterComparisonMinMagLinearMipPoint = 0x94,
    IRFilterComparisonMinMagMipLinear = 0x95,
    IRFilterComparisonAnisotropic = 0xd5,
    IRFilterMinimumMinMagMipPoint = 0x100,
    IRFilterMinimumMinMagPointMipLinear = 0x101,
    IRFilterMinimumMinPointMagLinearMipPoint = 0x104,
    IRFilterMinimumMinPointMagMipLinear = 0x105,
    IRFilterMinimumMinLinearMagMipPoint = 0x110,
    IRFilterMinimumMinLinearMagPointMipLinear = 0x111,
    IRFilterMinimumMinMagLinearMipPoint = 0x114,
    IRFilterMinimumMinMagMipLinear = 0x115,
    IRFilterMinimumAnisotropic = 0x155,
    IRFilterMaximumMinMagMipPoint = 0x180,
    IRFilterMaximumMinMagPointMipLinear = 0x181,
    IRFilterMaximumMinPointMagLinearMipPoint = 0x184,
    IRFilterMaximumMinPointMagMipLinear = 0x185,
    IRFilterMaximumMinLinearMagMipPoint = 0x190,
    IRFilterMaximumMinLinearMagPointMipLinear = 0x191,
    IRFilterMaximumMinMagLinearMipPoint = 0x194,
    IRFilterMaximumMinMagMipLinear = 0x195,
    IRFilterMaximumAnisotropic = 0x1d5,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRTextureAddressMode {
    IRTextureAddressModeWrap = 1,
    IRTextureAddressModeMirror = 2,
    IRTextureAddressModeClamp = 3,
    IRTextureAddressModeBorder = 4,
    IRTextureAddressModeMirrorOnce = 5,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRComparisonFunction {
    IRComparisonFunctionNever = 1,
    IRComparisonFunctionLess = 2,
    IRComparisonFunctionEqual = 3,
    IRComparisonFunctionLessEqual = 4,
    IRComparisonFunctionGreater = 5,
    IRComparisonFunctionNotEqual = 6,
    IRComparisonFunctionGreaterEqual = 7,
    IRComparisonFunctionAlways = 8,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRStaticBorderColor {
    IRStaticBorderColorTransparentBlack = 0,
    IRStaticBorderColorOpaqueBlack = 1,
    IRStaticBorderColorOpaqueWhite = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRStaticSamplerDescriptor {
    pub filter: IRFilter,
    pub address_u: IRTextureAddressMode,
    pub address_v: IRTextureAddressMode,
    pub address_w: IRTextureAddressMode,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: IRComparisonFunction,
    pub border_color: IRStaticBorderColor,
    pub min_lod: f32,
    pub max_lod: f32,
    pub shader_register: u32,
    pub register_space: u32,
    pub shader_visibility: IRShaderVisibility,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRRootSignatureFlags {
    IRRootSignatureFlagNone = 0,
    IRRootSignatureFlagAllowInputAssemblerInputLayout = 0x1,
    IRRootSignatureFlagDenyVertexShaderRootAccess = 0x2,
    IRRootSignatureFlagDenyHullShaderRootAccess = 0x4,
    IRRootSignatureFlagDenyDomainShaderRootAccess = 0x8,
    IRRootSignatureFlagDenyGeometryShaderRootAccess = 0x10,
    IRRootSignatureFlagDenyPixelShaderRootAccess = 0x20,
    IRRootSignatureFlagAllowStreamOutput = 0x40,
    IRRootSignatureFlagLocalRootSignature = 0x80,
    IRRootSignatureFlagDenyAmplificationShaderRootAccess = 0x100,
    IRRootSignatureFlagDenyMeshShaderRootAccess = 0x200,
    IRRootSignatureFlagCBVSRVUAVHeapDirectlyIndexed = 0x400,
    IRRootSignatureFlagSamplerHeapDirectlyIndexed = 0x800,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRRootSignatureDescriptor1 {
    pub num_parameters: u32,
    pub p_parameters: *const IRRootParameter1,
    pub num_static_samplers: u32,
    pub p_static_samplers: *const IRStaticSamplerDescriptor,
    pub flags: IRRootSignatureFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union IRVersionedRootSignatureDescriptor_u {
    // desc_1_0: IRRootSignatureDescriptor,
    pub desc_1_1: IRRootSignatureDescriptor1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IRRootSignatureVersion {
    IRRootSignatureVersion_1_0 = 0x1,
    IRRootSignatureVersion_1_1 = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRVersionedRootSignatureDescriptor {
    pub version: IRRootSignatureVersion,
    pub u: IRVersionedRootSignatureDescriptor_u,
}

#[derive(Debug)]
struct IRRootSignatureFn<'lib> {
    create_from_descriptor: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *const IRVersionedRootSignatureDescriptor,
            *mut *mut IRErrorOpaque,
        ) -> *mut IRRootSignatureOpaque,
    >,
    destroy: libloading::Symbol<'lib, unsafe extern "C" fn(*const IRRootSignatureOpaque) -> ()>,
    get_resource_count: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    get_resource_locations: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
}

pub struct IRRootSignature<'lib> {
    me: *mut IRRootSignatureOpaque,
    funcs: IRRootSignatureFn<'lib>,
}

impl<'lib> Drop for IRRootSignature<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRRootSignature<'lib> {
    pub fn create_from_descriptor(
        lib: &'lib libloading::Library,
        desc: &IRVersionedRootSignatureDescriptor,
    ) -> Result<IRRootSignature<'lib>, Box<dyn std::error::Error>> {
        unsafe {
            let funcs = IRRootSignatureFn {
                create_from_descriptor: lib.get(b"IRRootSignatureCreateFromDescriptor")?,
                destroy: lib.get(b"IRRootSignatureDestroy")?,
                get_resource_count: lib.get(b"IRRootSignatureGetResourceCount")?,
                get_resource_locations: lib.get(b"IRRootSignatureGetResourceLocations")?,
            };

            let mut error: *mut IRErrorOpaque =
                unsafe { std::ptr::null_mut::<IRErrorOpaque>().add(0) };

            let me = (funcs.create_from_descriptor)(desc, &mut error);

            dbg!(error);

            Ok(Self { funcs, me })
        }
    }
}

pub struct IRCompiler<'lib> {
    me: *mut IRCompilerOpaque,
    funcs: IRCompilerFn<'lib>,
}

impl<'lib> Drop for IRCompiler<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRCompiler<'lib> {
    pub fn new(
        lib: &'lib libloading::Library,
    ) -> Result<IRCompiler<'lib>, Box<dyn std::error::Error>> {
        unsafe {
            let funcs = IRCompilerFn {
                alloc_compile_and_link: lib.get(b"IRCompilerAllocCompileAndLink")?,
                create: lib.get(b"IRCompilerCreate")?,
                destroy: lib.get(b"IRCompilerDestroy")?,
                enable_geometry_and_tessellation_emulation: lib
                    .get(b"IRCompilerEnableGeometryAndTessellationEmulation")?,
                set_compatibility_flags: lib.get(b"IRCompilerSetCompatibilityFlags")?,
                set_depth_feedback_configuration: lib
                    .get(b"IRCompilerSetDepthFeedbackConfiguration")?,
                set_dual_source_blending_configuration: lib
                    .get(b"IRCompilerSetDualSourceBlendingConfiguration")?,
                set_entry_point_name: lib.get(b"IRCompilerSetEntryPointName")?,
                set_global_root_signature: lib.get(b"IRCompilerSetGlobalRootSignature")?,
                set_has_geometry_shader: lib.get(b"IRCompilerSetHasGeometryShader")?,
                set_hitgroup_arguments: lib.get(b"IRCompilerSetHitgroupArguments")?,
                set_hitgroup_type: lib.get(b"IRCompilerSetHitgroupType")?,
                set_input_topology: lib.get(b"IRCompilerSetInputTopology")?,
                set_local_root_signature: lib.get(b"IRCompilerSetLocalRootSignature")?,
                set_minimum_deployment_target: lib.get(b"IRCompilerSetMinimumDeploymentTarget")?,
                set_minimum_gpu_family: lib.get(b"IRCompilerSetMinimumGPUFamily")?,
                set_ray_tracing_pipeline_arguments: lib
                    .get(b"IRCompilerSetRayTracingPipelineArguments")?,
                set_shared_rt_arguments: lib.get(b"IRCompilerSetSharedRTArguments")?,
                set_stage_in_generation_mode: lib.get(b"IRCompilerSetStageInGenerationMode")?,
                set_stream_out_enabled: lib.get(b"IRCompilerSetStreamOutEnabled")?,
                set_strip_cut_index: lib.get(b"IRCompilerSetStripCutIndex")?,
                set_tessellation_enabled: lib.get(b"IRCompilerSetTessellationEnabled")?,
                set_tessellator_output_primitive: lib
                    .get(b"IRCompilerSetTessellatorOutputPrimitive")?,
                set_validation_flags: lib.get(b"IRCompilerSetValidationFlags")?,
                set_vertex_render_target_index_id: lib
                    .get(b"IRCompilerSetVertexRenderTargetIndexID")?,
                set_vertex_viewport_index_id: lib.get(b"IRCompilerSetVertexViewportIndexID")?,
                set_int_rt_mask: lib.get(b"IRCompilerSetIntRTMask")?,
                ignore_root_signature: lib.get(b"IRCompilerIgnoreRootSignature")?,
                synthesize_indirect_intersection_function: lib
                    .get(b"IRMetalLibSynthesizeIndirectIntersectionFunction")?,
            };

            let me = (funcs.create)();

            Ok(Self { funcs, me })
        }
    }

    pub fn set_global_root_signature(&mut self, root_signature: &IRRootSignature) {
        unsafe {
            (self.funcs.set_global_root_signature)(self.me, root_signature.me);
        }
    }

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
            (self.funcs.set_ray_tracing_pipeline_arguments)(
                self.me,
                max_attribute_size_in_bytes,
                raytracing_pipeline_flags,
                chs,
                miss,
                any_hit,
                callable_args,
                max_recursive_depth,
            );
        }
    }

    pub fn set_hitgroup_type(&mut self, hit_group_type: IRHitGroupType) {
        unsafe {
            (self.funcs.set_hitgroup_type)(self.me, hit_group_type);
        }
    }

    pub fn synthesize_indirect_intersection_function(
        &mut self,
        target_metallib: &mut IRMetalLibBinary,
    ) {
        unsafe {
            (self.funcs.synthesize_indirect_intersection_function)(self.me, target_metallib.me)
        }
    }

    pub fn alloc_compile_and_link(
        &mut self,
        entry_point: &[u8],
        input: &'lib IRObject,
    ) -> Result<IRObject<'lib>, Box<dyn std::error::Error>> {
        let mut error: *mut IRErrorOpaque = unsafe { std::ptr::null_mut::<IRErrorOpaque>().add(0) };

        let v = unsafe {
            (self.funcs.alloc_compile_and_link)(self.me, entry_point.as_ptr(), input.me, &mut error)
        };

        dbg!(v);

        dbg!(error);

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
