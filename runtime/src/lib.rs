#![doc = include_str!("../README.md")]

#[expect(
    clippy::missing_safety_doc,
    clippy::ptr_offset_with_cast,
    clippy::useless_transmute,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod bindings {
    // Use an include to be able to import more items into this module as below
    include!("bindings.rs");

    pub use objc2_metal::MTLResourceID;
}
pub use bindings as ffi;

use std::ptr::NonNull;

use objc2::runtime::ProtocolObject;
use objc2_metal::{
    MTL4ArgumentTable, MTL4RenderCommandEncoder, MTLBuffer, MTLGPUAddress, MTLIndexType,
    MTLPrimitiveType, MTLRenderCommandEncoder, MTLSamplerState, MTLTexture,
};

/// Rust version of `IRBufferView` using [`metal`] types.
#[doc(alias = "IRBufferView")]
pub struct BufferView<'a> {
    pub buffer: &'a ProtocolObject<dyn MTLBuffer>,
    pub buffer_offset: u64,
    pub buffer_size: u64,
    pub texture_buffer_view: Option<&'a ProtocolObject<dyn MTLTexture>>,
    pub texture_view_offset_in_elements: u32,
    pub typed_buffer: bool,
}

impl ffi::IRDescriptorTableEntry {
    /// Encode a buffer descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetBuffer` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    // TODO: This function seems to have no reason to exist, in favour of `buffer_view()` the
    // `metadata` argument here needs to be constructed in the exact same way.  However, for a
    // full buffer descriptor, setting the metadata to `0` seems to be fine?
    // TODO: The docs say  "buffer view" for metadata: can we take a BufferView struct and set
    // `Self::buffer_metadata()` instead? There are special constructors for atomic/counter buffers
    // after all...
    #[doc(alias = "IRDescriptorTableSetBuffer")]
    pub fn buffer(gpu_address: u64, metadata: u64) -> Self {
        Self {
            gpuVA: gpu_address,
            textureViewID: 0,
            metadata,
        }
    }

    /// Encode a buffer view descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetBufferView` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableSetBufferView")]
    pub fn buffer_view(buffer_view: &BufferView<'_>) -> Self {
        Self {
            gpuVA: buffer_view.buffer.gpuAddress() + buffer_view.buffer_offset,
            textureViewID: match buffer_view.texture_buffer_view {
                Some(texture) => unsafe { texture.gpuResourceID() }.to_raw(),
                None => 0,
            },
            metadata: Self::buffer_metadata(buffer_view),
        }
    }

    /// Encode a texture in this descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetTexture` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableSetTexture")]
    pub fn texture(argument: &ProtocolObject<dyn MTLTexture>, min_lod_clamp: f32) -> Self {
        const METADATA: u32 = 0; // According to the current docs, the metadata must be 0
        Self {
            gpuVA: 0,
            textureViewID: unsafe { argument.gpuResourceID() }.to_raw(),
            metadata: min_lod_clamp.to_bits() as u64 | ((METADATA as u64) << 32),
        }
    }

    /// Encode a sampler in this descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetSampler` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableSetSampler")]
    pub fn sampler(argument: &ProtocolObject<dyn MTLSamplerState>, lod_bias: f32) -> Self {
        Self {
            gpuVA: unsafe { argument.gpuResourceID() }.to_raw(),
            textureViewID: 0,
            metadata: lod_bias.to_bits() as u64,
        }
    }

    /// Encode an acceleration structure in this descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetAccelerationStructure` function in the `ir_raytracing.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableSetAccelerationStructure")]
    pub fn acceleration_structure(gpu_address: u64) -> Self {
        Self {
            gpuVA: gpu_address,
            textureViewID: 0,
            metadata: 0,
        }
    }

    /// Get the metadata value for a buffer view.
    ///
    /// This function is a port of the `IRDescriptorTableGetBufferMetadata` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableGetBufferMetadata")]
    pub fn buffer_metadata(view: &BufferView<'_>) -> u64 {
        let mut metadata = (view.buffer_size & ffi::kIRBufSizeMask) << ffi::kIRBufSizeOffset;
        metadata |= (view.texture_view_offset_in_elements as u64 & ffi::kIRTexViewMask)
            << ffi::kIRTexViewOffset;
        metadata |= (view.typed_buffer as u64) << ffi::kIRTypedBufferOffset;
        metadata
    }
}

/// Place these bytes in a buffer and return the GPU pointer to it, such that they stay alive
/// and unmodified until `encoder` completes on the GPU.
// TODO: We could also pass the binding index and let the user "set" the right
// binding directly on the `MTL4ArgumentTable` that they have bound for the `Vertex` stage.
pub type PushBytes = dyn FnMut(&[u8]) -> MTLGPUAddress;

#[doc(alias = "IRRuntimeDrawPrimitives")]
pub fn draw_primitives(
    encoder: &ProtocolObject<dyn MTLRenderCommandEncoder>,
    primitive_type: MTLPrimitiveType,
    vertex_start: usize,
    vertex_count: usize,
    instance_count: usize,
    base_instance: usize,
) {
    let mut dp = ffi::IRRuntimeDrawParams {
        u_1: ffi::IRRuntimeDrawParams_u {
            draw: ffi::IRRuntimeDrawArgument {
                vertexCountPerInstance: vertex_count as u32,
                instanceCount: instance_count as u32,
                startVertexLocation: vertex_start as u32,
                startInstanceLocation: base_instance as u32,
            },
        },
    };
    unsafe {
        encoder.setVertexBytes_length_atIndex(
            NonNull::new(&raw mut dp).unwrap().cast(),
            size_of_val(&dp),
            ffi::kIRArgumentBufferDrawArgumentsBindPoint as usize,
        );
        let mut non_indexed_draw = ffi::kIRNonIndexedDraw;
        encoder.setVertexBytes_length_atIndex(
            NonNull::new(&raw mut non_indexed_draw).unwrap().cast(),
            size_of_val(&non_indexed_draw),
            ffi::kIRArgumentBufferUniformsBindPoint as usize,
        );
        encoder.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
            primitive_type,
            vertex_start,
            vertex_count,
            instance_count,
            base_instance,
        );
    }
}

#[doc(alias = "IRRuntimeDrawPrimitives")]
pub fn mtl4_draw_primitives(
    encoder: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
    argument_table: &ProtocolObject<dyn MTL4ArgumentTable>,
    push_bytes: &mut PushBytes,
    primitive_type: MTLPrimitiveType,
    vertex_start: usize,
    vertex_count: usize,
    instance_count: usize,
    base_instance: usize,
) {
    let mut dp = ffi::IRRuntimeDrawParams {
        u_1: ffi::IRRuntimeDrawParams_u {
            draw: ffi::IRRuntimeDrawArgument {
                vertexCountPerInstance: vertex_count as u32,
                instanceCount: instance_count as u32,
                startVertexLocation: vertex_start as u32,
                startInstanceLocation: base_instance as u32,
            },
        },
    };
    unsafe {
        argument_table.setAddress_atIndex(
            push_bytes(&dp),
            // NonNull::new(&raw mut dp).unwrap().cast(),
            // size_of_val(&dp),
            ffi::kIRArgumentBufferDrawArgumentsBindPoint as usize,
        );
        let mut non_indexed_draw = ffi::kIRNonIndexedDraw;
        argument_table.setAddress_atIndex(
            push_bytes(&non_indexed_draw),
            // NonNull::new(&raw mut non_indexed_draw).unwrap().cast(),
            // size_of_val(&non_indexed_draw),
            ffi::kIRArgumentBufferUniformsBindPoint as usize,
        );
        encoder.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
            primitive_type,
            vertex_start,
            vertex_count,
            instance_count,
            base_instance,
        );
    }
}

#[doc(alias = "IRMetalIndexToIRIndex")]
pub fn metal_index_to_ir_index(index_type: MTLIndexType) -> u16 {
    index_type.0 as u16 + 1
}

#[doc(alias = "IRRuntimeDrawIndexedPrimitives")]
#[expect(clippy::too_many_arguments)]
pub fn draw_indexed_primitives(
    encoder: &ProtocolObject<dyn MTLRenderCommandEncoder>,
    primitive_type: MTLPrimitiveType,
    index_count: usize,
    index_type: MTLIndexType,
    index_buffer: &ProtocolObject<dyn MTLBuffer>,
    index_buffer_offset: usize,
    instance_count: usize,
    base_vertex: isize,
    base_instance: usize,
) {
    let mut dp = ffi::IRRuntimeDrawParams {
        u_1: ffi::IRRuntimeDrawParams_u {
            drawIndexed: ffi::IRRuntimeDrawIndexedArgument {
                indexCountPerInstance: index_count as u32,
                instanceCount: instance_count as u32,
                startIndexLocation: index_buffer_offset as u32,
                baseVertexLocation: base_vertex as i32,
                startInstanceLocation: base_instance as u32,
            },
        },
    };
    let mut ir_index_type = metal_index_to_ir_index(index_type);
    unsafe {
        encoder.setVertexBytes_length_atIndex(
            NonNull::new(&raw mut dp).unwrap().cast(),
            size_of_val(&dp),
            ffi::kIRArgumentBufferDrawArgumentsBindPoint as usize,
        );
        encoder.setVertexBytes_length_atIndex(
            NonNull::new(&raw mut ir_index_type).unwrap().cast(),
            size_of_val(&ir_index_type),
            ffi::kIRArgumentBufferUniformsBindPoint as usize,
        );
        encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
            primitive_type,
            index_count,
            index_type,
            index_buffer,
            index_buffer_offset,
            instance_count,
            base_vertex,
            base_instance,
        );
    }
}

#[doc(alias = "IRRuntimeDrawIndexedPrimitives")]
#[expect(clippy::too_many_arguments)]
pub fn mtl4_draw_indexed_primitives(
    encoder: &ProtocolObject<dyn MTL4RenderCommandEncoder>,
    argument_table: &ProtocolObject<dyn MTL4ArgumentTable>,
    push_bytes: &mut PushBytes,
    primitive_type: MTLPrimitiveType,
    index_count: usize,
    index_type: MTLIndexType,
    index_buffer: &ProtocolObject<dyn MTLBuffer>,
    index_buffer_offset: usize,
    instance_count: usize,
    base_vertex: isize,
    base_instance: usize,
) {
    // Assert that `argument_table` is bound to `encoder` for at least the vertex stage?

    let mut dp = ffi::IRRuntimeDrawParams {
        u_1: ffi::IRRuntimeDrawParams_u {
            drawIndexed: ffi::IRRuntimeDrawIndexedArgument {
                indexCountPerInstance: index_count as u32,
                instanceCount: instance_count as u32,
                startIndexLocation: index_buffer_offset as u32,
                baseVertexLocation: base_vertex as i32,
                startInstanceLocation: base_instance as u32,
            },
        },
    };
    let mut ir_index_type = metal_index_to_ir_index(index_type);
    unsafe {
        argument_table.setAddress_atIndex(
            push_bytes(&dp), // TODO: bytemuck?
            // NonNull::new(&raw mut dp).unwrap().cast(),
            // size_of_val(&dp),
            ffi::kIRArgumentBufferDrawArgumentsBindPoint as usize,
        );
        argument_table.setAddress_atIndex(
            push_bytes(&ir_index_type),
            // NonNull::new(&raw mut ir_index_type).unwrap().cast(),
            // size_of_val(&ir_index_type),
            ffi::kIRArgumentBufferUniformsBindPoint as usize,
        );
        encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_instanceCount_baseVertex_baseInstance(
            primitive_type,
            index_count,
            index_type,
            index_buffer.gpuAddress()+
            index_buffer_offset as u64,
        index_buffer.length() - index_buffer_offset, // TODO: instance_count times size!  This range is only used as validation
            instance_count,
            base_vertex,
            base_instance,
        );
    }
}

#[doc(alias = "IRRuntimeDrawIndexedPrimitives")]
pub fn draw_indexed_primitives_indirect(
    encoder: &ProtocolObject<dyn MTLRenderCommandEncoder>,
    primitive_type: MTLPrimitiveType,
    index_type: MTLIndexType,
    index_buffer: &ProtocolObject<dyn MTLBuffer>,
    index_buffer_offset: usize,
    indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
    indirect_buffer_offset: usize,
) {
    let mut ir_index_type = metal_index_to_ir_index(index_type);

    unsafe {
        encoder.setVertexBuffer_offset_atIndex(
            Some(indirect_buffer),
            0,
            ffi::kIRArgumentBufferDrawArgumentsBindPoint as usize,
        );
        encoder.setVertexBytes_length_atIndex(
            NonNull::new(&raw mut ir_index_type).unwrap().cast(),
            size_of_val(&ir_index_type),
            ffi::kIRArgumentBufferUniformsBindPoint as usize,
        );
        encoder.drawIndexedPrimitives_indexType_indexBuffer_indexBufferOffset_indirectBuffer_indirectBufferOffset(
            primitive_type,
            index_type,
            index_buffer,
            index_buffer_offset,
            indirect_buffer,
            indirect_buffer_offset
        );
    }
}
