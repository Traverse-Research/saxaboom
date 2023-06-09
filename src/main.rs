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

#[derive(Debug, Clone)]
struct IRObjectFn<'lib> {
    create_from_dxil: libloading::Symbol<
        'lib,
        unsafe extern "C" fn(*const u8, usize, IRBytecodeOwnership) -> *mut IRObjectOpaque,
    >,
    destroy: libloading::Symbol<'lib, unsafe extern "C" fn(*mut IRObjectOpaque) -> ()>,
    get_metal_ir_shader_stage: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    get_metal_lib_binary: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    get_reflection: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    get_type: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
    serialize: libloading::Symbol<'lib, unsafe extern "C" fn() -> ()>,
}

#[repr(u32)]
enum IRBytecodeOwnership {
    None = 0,
    Copy = 1,
}

struct IRObject<'lib> {
    me: *mut IRObjectOpaque,
    funcs: IRObjectFn<'lib>,
}

impl<'lib> Drop for IRObject<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRObject<'lib> {
    fn create_from_dxil(
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
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRDescriptorRangeType {
    IRDescriptorRangeTypeSRV = 0,
    IRDescriptorRangeTypeUAV = 1,
    IRDescriptorRangeTypeCBV = 2,
    IRDescriptorRangeTypeSampler = 3,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRDescriptorRangeFlags {
    IRDescriptorRangeFlagNone = 0,
    IRDescriptorRangeFlagDescriptorsVolatile = 0x1,
    IRDescriptorRangeFlagDataVolatile = 0x2,
    IRDescriptorRangeFlagDataStaticWhileSetAtExecute = 0x4,
    IRDescriptorRangeFlagDataStatic = 0x8,
    IRDescriptorRangeFlagDescriptorsStaticKeepingBufferBoundsChecks = 0x10000,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IRDescriptorRange1 {
    range_type: IRDescriptorRangeType,
    num_descriptors: u32,
    base_shader_register: u32,
    register_space: u32,
    flags: IRDescriptorRangeFlags,
    offset_in_descriptors_from_table_start: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IRRootDescriptorTable1 {
    num_descriptor_ranges: u32,
    p_descriptor_ranges: *const IRDescriptorRange1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRRootDescriptorFlags {
    IRRootDescriptorFlagNone = 0,
    IRRootDescriptorFlagDataVolatile = 0x2,
    IRRootDescriptorFlagDataStaticWhileSetAtExecute = 0x4,
    IRRootDescriptorFlagDataStatic = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IRRootDescriptor1 {
    shader_register: u32,
    register_space: u32,
    flags: IRRootDescriptorFlags,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRRootParameterType {
    IRRootParameterTypeDescriptorTable = 0,
    IRRootParameterType32BitConstants = 1,
    IRRootParameterTypeCBV = 2,
    IRRootParameterTypeSRV = 3,
    IRRootParameterTypeUAV = 4,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRShaderVisibility {
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
struct IRRootConstants {
    shader_register: u32,
    register_space: u32,
    num32_bit_values: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
union IRRootParameter1_u {
    descriptor_table: IRRootDescriptorTable1,
    constants: IRRootConstants,
    descriptor: IRRootDescriptor1,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IRRootParameter1 {
    parameter_type: IRRootParameterType,
    u: IRRootParameter1_u,
    shader_visibility: IRShaderVisibility,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRFilter {
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
enum IRTextureAddressMode {
    IRTextureAddressModeWrap = 1,
    IRTextureAddressModeMirror = 2,
    IRTextureAddressModeClamp = 3,
    IRTextureAddressModeBorder = 4,
    IRTextureAddressModeMirrorOnce = 5,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRComparisonFunction {
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
enum IRStaticBorderColor {
    IRStaticBorderColorTransparentBlack = 0,
    IRStaticBorderColorOpaqueBlack = 1,
    IRStaticBorderColorOpaqueWhite = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IRStaticSamplerDescriptor {
    filter: IRFilter,
    address_u: IRTextureAddressMode,
    address_v: IRTextureAddressMode,
    address_w: IRTextureAddressMode,
    mip_lod_bias: f32,
    max_anisotropy: u32,
    comparison_func: IRComparisonFunction,
    border_color: IRStaticBorderColor,
    min_lod: f32,
    max_lod: f32,
    shader_register: u32,
    register_space: u32,
    shader_visibility: IRShaderVisibility,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRRootSignatureFlags {
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
struct IRRootSignatureDescriptor1 {
    num_parameters: u32,
    p_parameters: *const IRRootParameter1,
    num_static_samplers: u32,
    p_static_samplers: *const IRStaticSamplerDescriptor,
    flags: IRRootSignatureFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
union IRVersionedRootSignatureDescriptor_u {
    // desc_1_0: IRRootSignatureDescriptor,
    desc_1_1: IRRootSignatureDescriptor1,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum IRRootSignatureVersion {
    IRRootSignatureVersion_1_0 = 0x1,
    IRRootSignatureVersion_1_1 = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct IRVersionedRootSignatureDescriptor {
    version: IRRootSignatureVersion,
    u: IRVersionedRootSignatureDescriptor_u,
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

struct IRRootSignature<'lib> {
    me: *mut IRRootSignatureOpaque,
    funcs: IRRootSignatureFn<'lib>,
}

impl<'lib> Drop for IRRootSignature<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

fn create_static_sampler(
    min_mag_mip_mode: IRFilter,
    address_mode: IRTextureAddressMode,
    index: u32,
    anisotropy: Option<u32>,
) -> IRStaticSamplerDescriptor {
    let max_anisotropy = anisotropy.unwrap_or(1);

    IRStaticSamplerDescriptor {
        filter: min_mag_mip_mode,
        address_u: address_mode,
        address_v: address_mode,
        address_w: address_mode,
        mip_lod_bias: 0.0,
        max_anisotropy: max_anisotropy,
        comparison_func: IRComparisonFunction::IRComparisonFunctionNever,
        min_lod: 0.0,
        max_lod: 100000.0,
        shader_register: index,
        register_space: 0,
        shader_visibility: IRShaderVisibility::IRShaderVisibilityAll,
        border_color: IRStaticBorderColor::IRStaticBorderColorTransparentBlack,
    }
}

impl<'lib> IRRootSignature<'lib> {
    fn create_from_descriptor(
        lib: &'lib libloading::Library,
        // bytecode: &[u8],
    ) -> Result<IRRootSignature<'lib>, Box<dyn std::error::Error>> {
        unsafe {
            let funcs = IRRootSignatureFn {
                create_from_descriptor: lib.get(b"IRRootSignatureCreateFromDescriptor")?,
                destroy: lib.get(b"IRRootSignatureDestroy")?,
                get_resource_count: lib.get(b"IRRootSignatureGetResourceCount")?,
                get_resource_locations: lib.get(b"IRRootSignatureGetResourceLocations")?,
            };

            let parameters = {
                let push_constants = IRRootParameter1 {
                    parameter_type: IRRootParameterType::IRRootParameterType32BitConstants,
                    shader_visibility: IRShaderVisibility::IRShaderVisibilityAll,
                    u: IRRootParameter1_u {
                        constants: IRRootConstants {
                            register_space: 0 as u32,
                            shader_register: 0,
                            num32_bit_values: 4, // debug has 6
                        },
                    },
                };

                let indirect_identifier = IRRootParameter1 {
                    parameter_type: IRRootParameterType::IRRootParameterType32BitConstants,
                    shader_visibility: IRShaderVisibility::IRShaderVisibilityAll,
                    u: IRRootParameter1_u {
                        constants: IRRootConstants {
                            register_space: 1 as u32,
                            shader_register: 0,
                            num32_bit_values: 1,
                        },
                    },
                };

                vec![push_constants, indirect_identifier]
            };

            let static_samplers = [
                create_static_sampler(
                    IRFilter::IRFilterMinMagMipPoint,
                    IRTextureAddressMode::IRTextureAddressModeWrap,
                    0,
                    None,
                ),
                create_static_sampler(
                    IRFilter::IRFilterMinMagMipPoint,
                    IRTextureAddressMode::IRTextureAddressModeClamp,
                    1,
                    None,
                ),
                create_static_sampler(
                    IRFilter::IRFilterMinMagMipLinear,
                    IRTextureAddressMode::IRTextureAddressModeWrap,
                    2,
                    None,
                ),
                create_static_sampler(
                    IRFilter::IRFilterMinMagMipLinear,
                    IRTextureAddressMode::IRTextureAddressModeClamp,
                    3,
                    None,
                ),
                create_static_sampler(
                    IRFilter::IRFilterMinMagMipLinear,
                    IRTextureAddressMode::IRTextureAddressModeBorder,
                    4,
                    None,
                ),
                create_static_sampler(
                    IRFilter::IRFilterAnisotropic,
                    IRTextureAddressMode::IRTextureAddressModeWrap,
                    5,
                    Some(2),
                ),
                create_static_sampler(
                    IRFilter::IRFilterAnisotropic,
                    IRTextureAddressMode::IRTextureAddressModeWrap,
                    6,
                    Some(4),
                ),
            ];

            let desc_1_1 = IRRootSignatureDescriptor1 {
                flags: IRRootSignatureFlags::IRRootSignatureFlagCBVSRVUAVHeapDirectlyIndexed,
                num_parameters: parameters.len() as u32,
                p_parameters: parameters.as_ptr(),
                num_static_samplers: static_samplers.len() as u32,
                p_static_samplers: static_samplers.as_ptr(),
            };

            let desc = IRVersionedRootSignatureDescriptor {
                version: IRRootSignatureVersion::IRRootSignatureVersion_1_1,
                u: IRVersionedRootSignatureDescriptor_u { desc_1_1 },
            };

            let mut error: *mut IRErrorOpaque =
                unsafe { std::ptr::null_mut::<IRErrorOpaque>().add(0) };

            let me = (funcs.create_from_descriptor)(&desc, &mut error);

            dbg!(error);

            Ok(Self { funcs, me })
        }
    }
}

struct IRCompiler<'lib> {
    me: *mut IRCompilerOpaque,
    funcs: IRCompilerFn<'lib>,
}

impl<'lib> Drop for IRCompiler<'lib> {
    fn drop(&mut self) {
        unsafe { (self.funcs.destroy)(self.me) }
    }
}

impl<'lib> IRCompiler<'lib> {
    fn new(lib: &'lib libloading::Library) -> Result<IRCompiler<'lib>, Box<dyn std::error::Error>> {
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

    fn set_global_root_signature(&mut self, root_signature: &IRRootSignature) {
        unsafe {
            (self.funcs.set_global_root_signature)(self.me, root_signature.me);
        }
    }

    fn alloc_compile_and_link(
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(
            "C:/Program Files/Metal Shader Converter/lib/metalirconverter.dll",
        )?;
        //let obj = IRObject::create_from_dxil(&lib, include_bytes!("C:/Users/Jasper/traverse/breda/apps/cs-memcpy/assets/shaders/memcpy.cs.dxil"))?;

        let root_sig = IRRootSignature::create_from_descriptor(&lib)?;

        let obj = IRObject::create_from_dxil(&lib, include_bytes!("C:/Users/Jasper/traverse/breda/crates/breda-egui/assets/shaders/egui_update.cs.dxil"))?;
        let mut c = IRCompiler::new(&lib)?;
        c.set_global_root_signature(&root_sig);
        c.alloc_compile_and_link(&[b"main\0"], &obj);
    }
    Ok(())
}
