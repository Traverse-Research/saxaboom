use std::ffi::CStr;

use saxaboom::{
    IRComparisonFunction, IRCompilerFactory, IRFilter, IRMetalLibBinary, IRObject,
    IRReflectionVersion, IRRootConstants, IRRootParameter1, IRRootParameter1_u,
    IRRootParameterType, IRRootSignature, IRRootSignatureDescriptor1, IRRootSignatureFlags,
    IRRootSignatureVersion, IRShaderReflection, IRShaderStage, IRShaderVisibility,
    IRStaticBorderColor, IRStaticSamplerDescriptor, IRTextureAddressMode,
    IRVersionedRootSignatureDescriptor, IRVersionedRootSignatureDescriptor_u,
};

fn create_static_sampler(
    min_mag_mip_mode: IRFilter,
    address_mode: IRTextureAddressMode,
    index: u32,
    anisotropy: Option<u32>,
) -> IRStaticSamplerDescriptor {
    let max_anisotropy = anisotropy.unwrap_or(1);

    IRStaticSamplerDescriptor {
        Filter: min_mag_mip_mode,
        AddressU: address_mode,
        AddressV: address_mode,
        AddressW: address_mode,
        MipLODBias: 0.0,
        MaxAnisotropy: max_anisotropy,
        ComparisonFunc: IRComparisonFunction::IRComparisonFunctionNever,
        MinLOD: 0.0,
        MaxLOD: 100000.0,
        ShaderRegister: index,
        RegisterSpace: 0,
        ShaderVisibility: IRShaderVisibility::IRShaderVisibilityAll,
        BorderColor: IRStaticBorderColor::IRStaticBorderColorTransparentBlack,
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

        let desc_1_1 = IRRootSignatureDescriptor1 {
            Flags: IRRootSignatureFlags::IRRootSignatureFlagCBVSRVUAVHeapDirectlyIndexed,
            NumParameters: parameters.len() as u32,
            pParameters: parameters.as_mut_ptr(),
            NumStaticSamplers: static_samplers.len() as u32,
            pStaticSamplers: static_samplers.as_mut_ptr(),
        };

        let desc = IRVersionedRootSignatureDescriptor {
            version: IRRootSignatureVersion::IRRootSignatureVersion_1_1,
            __bindgen_anon_1: IRVersionedRootSignatureDescriptor_u { desc_1_1 },
        };

        let root_sig = IRRootSignature::create_from_descriptor(&compiler, &desc)?;
        compiler.set_global_root_signature(&root_sig);

        // Load DXIL
        let dxil = include_bytes!("assets/memcpy.cs.dxil");
        let obj = IRObject::create_from_dxil(&compiler, dxil)?;

        // Convert to Metal
        let mut mtl_binary = IRMetalLibBinary::new(&compiler)?;
        let mtllib = compiler
            .alloc_compile_and_link(CStr::from_bytes_with_nul_unchecked(b"main\0"), &obj)?;
        mtllib.get_metal_lib_binary(IRShaderStage::IRShaderStageCompute, &mut mtl_binary);

        // Get Metal bytecode
        let metal_bytecode = mtl_binary.get_byte_code();
        dbg!(metal_bytecode.len());
        dbg!(mtllib.get_type());
        dbg!(mtllib.get_metal_ir_shader_stage());

        // Get reflection from the shader
        let mut mtl_reflection = IRShaderReflection::new(&compiler)?;
        mtllib.get_reflection(IRShaderStage::IRShaderStageCompute, &mut mtl_reflection);

        let compute_info = mtl_reflection
            .get_compute_info(IRReflectionVersion::IRReflectionVersion_1_0)
            .unwrap()
            .__bindgen_anon_1
            .info_1_0;
        dbg!(compute_info);
    }
    Ok(())
}

fn create_root_parameters() -> Vec<IRRootParameter1> {
    let push_constants = IRRootParameter1 {
        ParameterType: IRRootParameterType::IRRootParameterType32BitConstants,
        ShaderVisibility: IRShaderVisibility::IRShaderVisibilityAll,
        __bindgen_anon_1: IRRootParameter1_u {
            Constants: IRRootConstants {
                RegisterSpace: 0,
                ShaderRegister: 0,
                Num32BitValues: 7, // debug has 6
            },
        },
    };

    let indirect_identifier = IRRootParameter1 {
        ParameterType: IRRootParameterType::IRRootParameterType32BitConstants,
        ShaderVisibility: IRShaderVisibility::IRShaderVisibilityAll,
        __bindgen_anon_1: IRRootParameter1_u {
            Constants: IRRootConstants {
                RegisterSpace: 1,
                ShaderRegister: 0,
                Num32BitValues: 2,
            },
        },
    };

    vec![push_constants, indirect_identifier]
}

fn create_static_samplers() -> Vec<IRStaticSamplerDescriptor> {
    vec![
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
    ]
}
