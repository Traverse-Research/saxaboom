use saxaboom::{
    IRComparisonFunction, IRCompiler, IRFilter, IRMetalLibBinary, IRObject, IRRootConstants,
    IRRootDescriptor1, IRRootParameter1, IRRootParameter1_u, IRRootParameterType, IRRootSignature,
    IRRootSignatureDescriptor1, IRRootSignatureFlags, IRRootSignatureVersion, IRShaderStage,
    IRShaderVisibility, IRStaticBorderColor, IRStaticSamplerDescriptor, IRTextureAddressMode,
    IRVersionedRootSignatureDescriptor, IRVersionedRootSignatureDescriptor_u,
};

use saxaboom::bindings::*;

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
        let lib = libloading::Library::new(
            "C:/Program Files/Metal Shader Converter/lib/metalirconverter.dll",
        )?;

        let parameters = {
            let push_constants = IRRootParameter1 {
                ParameterType: IRRootParameterType::IRRootParameterType32BitConstants,
                ShaderVisibility: IRShaderVisibility::IRShaderVisibilityAll,
                u_1: IRRootParameter1_u {
                    Constants: IRRootConstants {
                        RegisterSpace: 0 as u32,
                        ShaderRegister: 0,
                        Num32BitValues: 4, // debug has 6
                    },
                },
            };

            let indirect_identifier = IRRootParameter1 {
                ParameterType: IRRootParameterType::IRRootParameterType32BitConstants,
                ShaderVisibility: IRShaderVisibility::IRShaderVisibilityAll,
                u_1: IRRootParameter1_u {
                    Constants: IRRootConstants {
                        RegisterSpace: 1 as u32,
                        ShaderRegister: 0,
                        Num32BitValues: 1,
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
            Flags: IRRootSignatureFlags::IRRootSignatureFlagCBVSRVUAVHeapDirectlyIndexed,
            NumParameters: parameters.len() as u32,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: static_samplers.len() as u32,
            pStaticSamplers: static_samplers.as_ptr(),
        };

        let desc = IRVersionedRootSignatureDescriptor {
            version: IRRootSignatureVersion::IRRootSignatureVersion_1_1,
            u_1: IRVersionedRootSignatureDescriptor_u { desc_1_1 },
        };

        let root_sig = IRRootSignature::create_from_descriptor(&lib, &desc)?;

        let egui_update = include_bytes!(
            "C:/Users/Jasper/traverse/breda/crates/breda-egui/assets/shaders/egui_update.cs.dxil"
        );
        // let memcpy = include_bytes!("C:/Users/Jasper/traverse/breda/apps/cs-memcpy/assets/shaders/memcpy.cs.dxil");

        let mut mtl_binary = IRMetalLibBinary::new(&lib)?;

        let obj = IRObject::create_from_dxil(&lib, egui_update)?;
        let mut c = IRCompiler::new(&lib)?;
        c.set_global_root_signature(&root_sig);
        let mtllib = c.alloc_compile_and_link(&[b"main\0"], &obj)?;
        dbg!(mtllib.get_type());
        dbg!(mtllib.get_metal_ir_shader_stage());
        mtllib.get_metal_lib_binary(IRShaderStage::IRShaderStageCompute, &mut mtl_binary);
        dbg!(mtl_binary.get_byte_code().len());
        std::fs::write("out.bin", mtl_binary.get_byte_code());
    }

    {
        use saxaboom::bindings::*;
        use saxaboom::types::*;

        let lib = unsafe {
            MetalIrConverter::new(
                "C:/Program Files/Metal Shader Converter/lib/metalirconverter.dll",
            )?
        };

        dbg!(lib.IRErrorGetCode);
    }
    Ok(())
}
