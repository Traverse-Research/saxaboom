struct IRCompilerOpaque;
struct IRObjectOpaque;
struct IRRootSignatureOpaque;
struct IRMetalLibBinaryOpaque;
struct IRShaderReflectionOpaque;
struct IRErrorOpaque;

#[derive(Debug)]
struct IRCompilerFn<'lib> {
    alloc_compile_and_link: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(
            *mut IRCompilerOpaque,
            *const *const u8,
            entry_points: usize,
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
}

#[repr(u32)]
#[derive(Debug)]
pub enum IRObjectType {
    IRObjectTypeDXILBytecode,
    IRObjectTypeMetalIRObject,
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
    get_reflection: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    get_type: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRObjectOpaque) -> IRObjectType>,
    serialize: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
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
                set_input_topology: lib.get(b"IRCompilerSetInputTopology")?,
                set_local_root_signature: lib.get(b"IRCompilerSetLocalRootSignature")?,
                set_minimum_deployment_target: lib.get(b"IRCompilerSetMinimumDeploymentTarget")?,
                set_minimum_gpu_family: lib.get(b"IRCompilerSetMinimumGPUFamily")?,
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

    pub fn alloc_compile_and_link(
        &mut self,
        entry_points: &[&[u8]],
        input: &'lib IRObject,
    ) -> Result<IRObject<'lib>, Box<dyn std::error::Error>> {
        let entry_points = entry_points
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<*const u8>>();
        let mut error: *mut IRErrorOpaque = unsafe { std::ptr::null_mut::<IRErrorOpaque>().add(0) };

        let v = unsafe {
            (self.funcs.alloc_compile_and_link)(
                self.me,
                entry_points.as_ptr(),
                entry_points.len(),
                input.me,
                &mut error,
            )
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
