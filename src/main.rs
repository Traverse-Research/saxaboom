use saxaboom::{
    ffi, IRCompilerFactory, IRMetalLibBinary, IRObject, IRRootSignature, IRShaderReflection,
};

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
        ComparisonFunc: ffi::IRComparisonFunction::IRComparisonFunctionNever,
        MinLOD: 0.0,
        MaxLOD: 100000.0,
        ShaderRegister: index,
        RegisterSpace: 0,
        ShaderVisibility: ffi::IRShaderVisibility::IRShaderVisibilityAll,
        BorderColor: ffi::IRStaticBorderColor::IRStaticBorderColorTransparentBlack,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Create an instance of IRCompiler
        let compiler_factory = IRCompilerFactory::new("libmetalirconverter.dylib").unwrap();
        let mut compiler = compiler_factory.create_compiler();

        // Create an explicit root signature layout
        let mut parameters = create_root_parameters();
        let mut static_samplers = create_static_samplers();

        let desc_1_1 = ffi::IRRootSignatureDescriptor1 {
            Flags: ffi::IRRootSignatureFlags::IRRootSignatureFlagCBVSRVUAVHeapDirectlyIndexed,
            NumParameters: parameters.len() as u32,
            pParameters: parameters.as_mut_ptr(),
            NumStaticSamplers: static_samplers.len() as u32,
            pStaticSamplers: static_samplers.as_mut_ptr(),
        };

        let desc = ffi::IRVersionedRootSignatureDescriptor {
            version: ffi::IRRootSignatureVersion::IRRootSignatureVersion_1_1,
            u_1: ffi::IRVersionedRootSignatureDescriptor_u { desc_1_1 },
        };

        let root_sig = IRRootSignature::create_from_descriptor(&compiler, &desc)?;
        compiler.set_global_root_signature(&root_sig);

        // Load DXIL
        let dxil = include_bytes!("assets/memcpy.cs.dxil");
        let obj = IRObject::create_from_dxil(&compiler, dxil)?;

        // Convert to Metal
        let mut mtl_binary = IRMetalLibBinary::new(&compiler)?;
        let mtllib = compiler.alloc_compile_and_link(c"main", &obj)?;
        mtllib.get_metal_lib_binary(ffi::IRShaderStage::IRShaderStageCompute, &mut mtl_binary);

        // Get Metal bytecode
        let metal_bytecode = mtl_binary.get_byte_code();
        dbg!(metal_bytecode.len());
        dbg!(mtllib.get_type());
        dbg!(mtllib.get_metal_ir_shader_stage());

        // Get reflection from the shader
        let mut mtl_reflection = IRShaderReflection::new(&compiler)?;
        mtllib.get_reflection(
            ffi::IRShaderStage::IRShaderStageCompute,
            &mut mtl_reflection,
        );

        let compute_info = mtl_reflection
            .get_compute_info(ffi::IRReflectionVersion::IRReflectionVersion_1_0)
            .unwrap()
            .u_1
            .info_1_0;
        dbg!(compute_info);
    }
    Ok(())
}

fn create_root_parameters() -> Vec<ffi::IRRootParameter1> {
    let push_constants = ffi::IRRootParameter1 {
        ParameterType: ffi::IRRootParameterType::IRRootParameterType32BitConstants,
        ShaderVisibility: ffi::IRShaderVisibility::IRShaderVisibilityAll,
        u_1: ffi::IRRootParameter1_u {
            Constants: ffi::IRRootConstants {
                RegisterSpace: 0,
                ShaderRegister: 0,
                Num32BitValues: 7, // debug has 6
            },
        },
    };

    let indirect_identifier = ffi::IRRootParameter1 {
        ParameterType: ffi::IRRootParameterType::IRRootParameterType32BitConstants,
        ShaderVisibility: ffi::IRShaderVisibility::IRShaderVisibilityAll,
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
            ffi::IRFilter::IRFilterMinMagMipPoint,
            ffi::IRTextureAddressMode::IRTextureAddressModeWrap,
            0,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::IRFilterMinMagMipPoint,
            ffi::IRTextureAddressMode::IRTextureAddressModeClamp,
            1,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::IRFilterMinMagMipLinear,
            ffi::IRTextureAddressMode::IRTextureAddressModeWrap,
            2,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::IRFilterMinMagMipLinear,
            ffi::IRTextureAddressMode::IRTextureAddressModeClamp,
            3,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::IRFilterMinMagMipLinear,
            ffi::IRTextureAddressMode::IRTextureAddressModeBorder,
            4,
            None,
        ),
        create_static_sampler(
            ffi::IRFilter::IRFilterAnisotropic,
            ffi::IRTextureAddressMode::IRTextureAddressModeWrap,
            5,
            Some(2),
        ),
        create_static_sampler(
            ffi::IRFilter::IRFilterAnisotropic,
            ffi::IRTextureAddressMode::IRTextureAddressModeWrap,
            6,
            Some(4),
        ),
    ]
}
