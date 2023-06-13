use saxaboom::{
    IRComparisonFunction, IRCompiler, IRFilter, IRMetalLibBinary, IRObject, IRRootConstants,
    IRRootDescriptor1, IRRootParameter1, IRRootParameter1_u, IRRootParameterType, IRRootSignature,
    IRRootSignatureDescriptor1, IRRootSignatureFlags, IRRootSignatureVersion, IRShaderStage,
    IRShaderVisibility, IRStaticBorderColor, IRStaticSamplerDescriptor, IRTextureAddressMode,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new(
            "C:/Program Files/Metal Shader Converter/lib/metalirconverter.dll",
        )?;

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
    Ok(())
}
