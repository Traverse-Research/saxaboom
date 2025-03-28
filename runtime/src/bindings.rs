/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte =
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
pub type uint = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    pub threadgroupsPerGrid: [u32; 3usize],
}
pub type resourceid_t = MTLResourceID;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRDescriptorTableEntry {
    pub gpuVA: u64,
    pub textureViewID: u64,
    pub metadata: u64,
}
pub const kIRRuntimeTessellatorTablesBindPoint: u64 = 7;
pub const kIRRuntimeTessellatorTablesCountsAndOffsetLength: u32 = 32768;
pub const kIRRuntimeTessellatorTablesLookupTableLength: u32 = 701114;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRShaderIdentifier {
    pub intersectionShaderHandle: u64,
    pub shaderHandle: u64,
    pub localRootSignatureSamplersBuffer: u64,
    pub pad0: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRVirtualAddressRange {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRVirtualAddressRangeAndStride {
    pub StartAddress: u64,
    pub SizeInBytes: u64,
    pub StrideInBytes: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRDispatchRaysDescriptor {
    pub RayGenerationShaderRecord: IRVirtualAddressRange,
    pub MissShaderTable: IRVirtualAddressRangeAndStride,
    pub HitGroupTable: IRVirtualAddressRangeAndStride,
    pub CallableShaderTable: IRVirtualAddressRangeAndStride,
    pub Width: uint,
    pub Height: uint,
    pub Depth: uint,
}
#[repr(C)]
pub struct IRDispatchRaysArgument {
    pub DispatchRaysDesc: IRDispatchRaysDescriptor,
    pub GRS: u64,
    pub ResDescHeap: u64,
    pub SmpDescHeap: u64,
    pub VisibleFunctionTable: resourceid_t,
    pub IntersectionFunctionTable: resourceid_t,
    pub IntersectionFunctionTables: u64,
}
impl Default for IRDispatchRaysArgument {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type dispatchthreadgroupsindirectargs_t = MTLDispatchThreadgroupsIndirectArguments;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRaytracingAccelerationStructureGPUHeader {
    pub accelerationStructureID: u64,
    pub addressOfInstanceContributions: u64,
    pub pad0: [u64; 4usize],
    pub pad1: dispatchthreadgroupsindirectargs_t,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRaytracingInstanceDescriptor {
    pub Transform: [[f32; 4usize]; 3usize],
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
    pub AccelerationStructure: u64,
}
impl IRRaytracingInstanceDescriptor {
    #[inline]
    pub fn InstanceID(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
    }
    #[inline]
    pub fn set_InstanceID(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn InstanceID_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                24u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_InstanceID_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                24u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn InstanceMask(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_InstanceMask(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn InstanceMask_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                24usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_InstanceMask_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                24usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn InstanceContributionToHitGroupIndex(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 24u8) as u32) }
    }
    #[inline]
    pub fn set_InstanceContributionToHitGroupIndex(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn InstanceContributionToHitGroupIndex_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                32usize,
                24u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_InstanceContributionToHitGroupIndex_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                32usize,
                24u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn Flags(&self) -> u32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(56usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_Flags(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(56usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn Flags_raw(this: *const Self) -> u32 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                56usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_Flags_raw(this: *mut Self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                56usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        InstanceID: u32,
        InstanceMask: u32,
        InstanceContributionToHitGroupIndex: u32,
        Flags: u32,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 24u8, {
            let InstanceID: u32 = unsafe { ::std::mem::transmute(InstanceID) };
            InstanceID as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let InstanceMask: u32 = unsafe { ::std::mem::transmute(InstanceMask) };
            InstanceMask as u64
        });
        __bindgen_bitfield_unit.set(32usize, 24u8, {
            let InstanceContributionToHitGroupIndex: u32 =
                unsafe { ::std::mem::transmute(InstanceContributionToHitGroupIndex) };
            InstanceContributionToHitGroupIndex as u64
        });
        __bindgen_bitfield_unit.set(56usize, 8u8, {
            let Flags: u32 = unsafe { ::std::mem::transmute(Flags) };
            Flags as u64
        });
        __bindgen_bitfield_unit
    }
}
pub const kIRRayDispatchIndirectionKernelName: &[u8; 18] = b"RaygenIndirection\0";
pub const kIRRayDispatchArgumentsBindPoint: u64 = 3;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IRRuntimeResourceType {
    SRV = 0,
    UAV = 1,
    CBV = 2,
    SMP = 3,
    Count = 4,
}
impl IRRuntimePrimitiveType {
    pub const _1ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _2ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _3ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _4ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _5ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _6ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _7ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _8ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _9ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _10ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _11ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _12ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _13ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _14ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _15ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _16ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _17ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _18ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _19ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _20ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _21ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _22ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _23ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _24ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _25ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _26ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _27ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _28ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _29ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _30ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _31ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
impl IRRuntimePrimitiveType {
    pub const _32ControlPointPatchlist: IRRuntimePrimitiveType = IRRuntimePrimitiveType::Triangle;
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IRRuntimePrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
    LineWithAdj = 5,
    TriangleWithAdj = 6,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRuntimeGeometryPipelineConfig {
    pub gsVertexSizeInBytes: u32,
    pub gsMaxInputPrimitivesPerMeshThreadgroup: u32,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IRRuntimeTessellatorOutputPrimitive {
    Undefined = 0,
    Point = 1,
    Line = 2,
    TriangleCW = 3,
    TriangleCCW = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IRRuntimeTessellationPipelineConfig {
    pub outputPrimitiveType: IRRuntimeTessellatorOutputPrimitive,
    pub vsOutputSizeInBytes: u32,
    pub gsMaxInputPrimitivesPerMeshThreadgroup: u32,
    pub hsMaxPatchesPerObjectThreadgroup: u32,
    pub hsInputControlPointCount: u32,
    pub hsMaxObjectThreadsPerThreadgroup: u32,
    pub hsMaxTessellationFactor: f32,
    pub gsInstanceCount: u32,
}
impl Default for IRRuntimeTessellationPipelineConfig {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRuntimeVertexBuffer {
    pub addr: u64,
    pub length: u32,
    pub stride: u32,
}
pub type IRRuntimeVertexBuffers = [IRRuntimeVertexBuffer; 31usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRuntimeDrawArgument {
    pub vertexCountPerInstance: uint,
    pub instanceCount: uint,
    pub startVertexLocation: uint,
    pub startInstanceLocation: uint,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRuntimeDrawIndexedArgument {
    pub indexCountPerInstance: uint,
    pub instanceCount: uint,
    pub startIndexLocation: uint,
    pub baseVertexLocation: ::std::os::raw::c_int,
    pub startInstanceLocation: uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IRRuntimeDrawParams {
    pub u_1: IRRuntimeDrawParams_u,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union IRRuntimeDrawParams_u {
    pub draw: IRRuntimeDrawArgument,
    pub drawIndexed: IRRuntimeDrawIndexedArgument,
}
impl Default for IRRuntimeDrawParams_u {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Default for IRRuntimeDrawParams {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IRRuntimeDrawInfo {
    pub indexType: u16,
    pub primitiveTopology: u8,
    pub threadsPerPatch: u8,
    pub maxInputPrimitivesPerMeshThreadgroup: u16,
    pub objectThreadgroupVertexStride: u16,
    pub meshThreadgroupPrimitiveStride: u16,
    pub gsInstanceCount: u16,
    pub patchesPerObjectThreadgroup: u16,
    pub inputControlPointsPerPatch: u16,
    pub indexBuffer: u64,
}
pub const kIRArgumentBufferBindPoint: u64 = 2;
pub const kIRArgumentBufferHullDomainBindPoint: u64 = 3;
pub const kIRDescriptorHeapBindPoint: u64 = 0;
pub const kIRSamplerHeapBindPoint: u64 = 1;
pub const kIRArgumentBufferDrawArgumentsBindPoint: u64 = 4;
pub const kIRArgumentBufferUniformsBindPoint: u64 = 5;
pub const kIRVertexBufferBindPoint: u64 = 6;
pub const kIRStageInAttributeStartIndex: u64 = 11;
pub const kIRIndirectTriangleIntersectionFunctionName: &[u8; 51] =
    b"irconverter.wrapper.intersection.function.triangle\0";
pub const kIRIndirectProceduralIntersectionFunctionName: &[u8; 53] =
    b"irconverter.wrapper.intersection.function.procedural\0";
pub const kIRTrianglePassthroughGeometryShader: &[u8; 47] =
    b"irconverter_domain_shader_triangle_passthrough\0";
pub const kIRLinePassthroughGeometryShader: &[u8; 43] =
    b"irconverter_domain_shader_line_passthrough\0";
pub const kIRPointPassthroughGeometryShader: &[u8; 44] =
    b"irconverter_domain_shader_point_passthrough\0";
pub const kIRNonIndexedDraw: u16 = 0;
pub const kIRFunctionGroupRayGeneration: &[u8; 7] = b"rayGen\0";
pub const kIRFunctionGroupClosestHit: &[u8; 11] = b"closestHit\0";
pub const kIRFunctionGroupMiss: &[u8; 5] = b"miss\0";
pub const kIRBufSizeOffset: u64 = 0;
pub const kIRBufSizeMask: u64 = 4294967295;
pub const kIRTexViewOffset: u64 = 32;
pub const kIRTexViewMask: u64 = 255;
pub const kIRTypedBufferOffset: u64 = 63;
