#![doc = include_str!("../README.md")]

pub mod bindings;
pub mod types;

pub use types::*;

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
                IRBytecodeOwnership::IRBytecodeOwnershipNone,
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
