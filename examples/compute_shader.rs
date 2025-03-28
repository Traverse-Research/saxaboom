use saxaboom::{ffi, MetalIrConverter};

fn create_static_sampler(
    min_mag_mip_mode: ffi::IRFilter,
    address_mode: ffi::IRTextureAddressMode,
    index: u32,
    anisotropy: Option<u32>,
) -> ffi::IRStaticSamplerDescriptor {
    let max_anisotropy = anisotropy.unwrap_or(1);

    ffi::IRStaticSamplerDescriptor {
        Filter: min_mag_mip_mode,
        AddressU: address_mode,
        AddressV: address_mode,
        AddressW: address_mode,
        MipLODBias: 0.0,
        MaxAnisotropy: max_anisotropy,
        ComparisonFunc: ffi::IRComparisonFunction::Never,
        MinLOD: 0.0,
        MaxLOD: 100000.0,
        ShaderRegister: index,
        RegisterSpace: 0,
        ShaderVisibility: ffi::IRShaderVisibility::All,
        BorderColor: ffi::IRStaticBorderColor::OpaqueBlack,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the library
    let metal_irconverter =
        MetalIrConverter::new(libloading::library_filename("metalirconverter")).unwrap();
    // Create an instance of IRCompiler
    let mut compiler = metal_irconverter.create_compiler();

    // Create an explicit root signature layout
    let mut parameters = create_root_parameters();
    let mut static_samplers = create_static_samplers();

    let desc_1_1 = ffi::IRRootSignatureDescriptor1 {
        Flags: ffi::IRRootSignatureFlags::CBVSRVUAVHeapDirectlyIndexed,
        NumParameters: parameters.len() as u32,
        pParameters: parameters.as_mut_ptr(),
        NumStaticSamplers: static_samplers.len() as u32,
        pStaticSamplers: static_samplers.as_mut_ptr(),
    };

    let desc = ffi::IRVersionedRootSignatureDescriptor {
        version: ffi::IRRootSignatureVersion::_1_1,
        u_1: ffi::IRVersionedRootSignatureDescriptor_u { desc_1_1 },
    };

    let root_sig = metal_irconverter.create_root_signature_from_descriptor(&desc)?;
    compiler.set_global_root_signature(&root_sig);

    // Load DXIL
    let dxil = include_bytes!("assets/memcpy.cs.dxil");
    let dxil = metal_irconverter.create_object_from_dxil(dxil);

    // Convert to Metal
    let mtllib = compiler.alloc_compile_and_link(c"main", &dxil)?;
    let mtl_binary = mtllib
        .metal_lib_binary()
        .expect("Compiled object should contain a `metallib`");

    // Get Metal bytecode
    let metal_bytecode = mtl_binary.byte_code();
    dbg!(metal_bytecode.len());
    dbg!(mtllib.r#type());
    dbg!(mtllib.metal_ir_shader_stage());

    // Get reflection from the shader
    let mtl_reflection = mtllib.reflection();

    let compute_info = mtl_reflection.map(|mtl_reflection| unsafe {
        mtl_reflection
            .compute_info(ffi::IRReflectionVersion::_1_0)
            .unwrap()
            .u_1
            .info_1_0
    });
    dbg!(compute_info);

    Ok(())
}

fn create_root_parameters() -> Vec<ffi::IRRootParameter1> {
    let push_constants = ffi::IRRootParameter1 {
        ParameterType: ffi::IRRootParameterType::_32BitConstants,
        ShaderVisibility: ffi::IRShaderVisibility::All,
        u_1: ffi::IRRootParameter1_u {
            Constants: ffi::IRRootConstants {
                RegisterSpace: 0,
                ShaderRegister: 0,
                Num32BitValues: 7,
            },
        },
    };

    let indirect_identifier = ffi::IRRootParameter1 {
        ParameterType: ffi::IRRootParameterType::_32BitConstants,
        ShaderVisibility: ffi::IRShaderVisibility::All,
        u_1: ffi::IRRootParameter1_u {
            Constants: ffi::IRRootConstants {
                RegisterSpace: 1,
                ShaderRegister: 0,
                Num32BitValues: 2,
            },
        },
    };

    vec![push_constants, indirect_identifier]
}

fn create_static_samplers() -> Vec<ffi::IRStaticSamplerDescriptor> {
    vec![
        create_static_sampler(
            ffi::IRFilter::MinMagMipPoint,
            ffi::IRTextureAddressMode::Wrap,
            0,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::MinMagMipPoint,
            ffi::IRTextureAddressMode::Clamp,
            1,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::MinMagMipLinear,
            ffi::IRTextureAddressMode::Wrap,
            2,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::MinMagMipLinear,
            ffi::IRTextureAddressMode::Clamp,
            3,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::MinMagMipLinear,
            ffi::IRTextureAddressMode::Border,
            4,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::Anisotropic,
            ffi::IRTextureAddressMode::Wrap,
            5,
            Some(2),
        ),
        create_static_sampler(
            ffi::IRFilter::Anisotropic,
            ffi::IRTextureAddressMode::Wrap,
            6,
            Some(4),
        ),
    ]
}
