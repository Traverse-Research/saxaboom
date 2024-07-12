//! Contains `repr(C)` definitions for structures used at runtime.

#[allow(
    clippy::useless_transmute,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod bindings {
    // Use an include to be able to import more items into this module as below
    include!("bindings.rs");

    pub use metal::MTLResourceID;
}
pub use bindings as ffi;

/// Rust version of `IRBufferView` using [`metal`] types.
#[doc(alias = "IRBufferView")]
pub struct BufferView<'a> {
    pub buffer: &'a metal::Buffer,
    pub buffer_offset: u64,
    pub buffer_size: u64,
    pub texture_buffer_view: Option<&'a metal::Texture>,
    pub texture_view_offset_in_elements: u32,
    pub typed_buffer: bool,
}

impl ffi::IRDescriptorTableEntry {
    /// Encode a buffer descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetBuffer` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    // TODO: The docs say  "buffer view" for metadata: can we take a BufferView struct and set `Self::buffer_metadata()` instead?
    // There are special constructors for atomic/counter buffers after all...
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
            gpuVA: buffer_view.buffer.gpu_address() + buffer_view.buffer_offset,
            textureViewID: match buffer_view.texture_buffer_view {
                Some(texture) => texture.gpu_resource_id()._impl,
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
    pub fn texture(argument: &metal::Texture, min_lod_clamp: f32) -> Self {
        const METADATA: u32 = 0; // According to the current docs, the metadata must be 0
        Self {
            gpuVA: 0,
            textureViewID: argument.gpu_resource_id()._impl,
            metadata: min_lod_clamp.to_bits() as u64 | (METADATA as u64) << 32,
        }
    }

    /// Encode a sampler in this descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetSampler` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableSetSampler")]
    #[allow(unused_variables, unreachable_code, dead_code)]
    // TODO: Expose this function when metal-rs contains the update
    /* pub */
    fn sampler(argument: &metal::SamplerState, lod_bias: f32) -> Self {
        Self {
            gpuVA: todo!("Add gpu_resource_id() to SamplerState: https://github.com/gfx-rs/metal-rs/pull/328"), // argument.gpu_resource_id()._impl,
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
