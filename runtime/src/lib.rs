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

    // TODO: Use the proper type from one of the objc or objc2 crates?
    enum NSError {}

    use metal::{
        MTLIndexType, MTLMeshRenderPipelineDescriptor, MTLPrimitiveType, MTLResourceID, NSUInteger,
    };

    // TODO: Blocklist breaks generation of this type, and it's not in the metal crate either.
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct MTLDispatchThreadgroupsIndirectArguments {
        pub threadgroupsPerGrid: [u32; 3usize],
    }
}
use std::mem::MaybeUninit;

pub use bindings as ffi;

/// Rust version of `IRBufferView` using [`metal`] types.
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
    pub fn buffer(lib: &ffi::metal_irconverter, gpu_address: u64, metadata: u64) -> Self {
        let mut e = MaybeUninit::uninit();
        unsafe { (lib.IRDescriptorTableSetBuffer)(e.as_mut_ptr(), gpu_address, metadata) };
        unsafe { e.assume_init() }
    }

    /// Encode a buffer view descriptor.
    ///
    /// This function is a port of the `IRDescriptorTableSetBufferView` function in the `metal_irconverter_runtime.h` header.
    /// See <https://developer.apple.com/metal/shader-converter/> for more info.
    #[doc(alias = "IRDescriptorTableSetBufferView")]
    pub fn buffer_view(lib: &ffi::metal_irconverter, buffer_view: &BufferView<'_>) -> Self {
        let mut e = MaybeUninit::uninit();
        // TODO: No mut
        let mut buffer_view = ffi::IRBufferView {
            buffer: buffer_view.buffer.clone(),
            bufferOffset: buffer_view.buffer_offset,
            bufferSize: buffer_view.buffer_size,
            textureBufferView: buffer_view.texture_buffer_view.cloned(),
            textureViewOffsetInElements: buffer_view.texture_view_offset_in_elements,
            typedBuffer: buffer_view.typed_buffer,
        };
        unsafe { (lib.IRDescriptorTableSetBufferView)(e.as_mut_ptr(), &mut buffer_view) };
        unsafe { e.assume_init() }
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
