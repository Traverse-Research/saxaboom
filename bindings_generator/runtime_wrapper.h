#include <cstdint>
#include <cstdlib>

// Defines from MTLDefines.h
#define MTL_EXTERN
#define API_AVAILABLE(...)

// Fake forward declares for the runtime header - unfortunately raw structures
// and constants are completely intertwined with obj-c definitions in the Metal
// framework headers.
// All these declarations (and two definitions because their "layout" needs to
// be known) are irrelevant: they are needed for clang, but blocklisted in the
// generator script as we will replace them with proper Rust types.

typedef struct NSError_ NSError;
typedef struct NSString_ NSString;
typedef size_t NSUInteger;
template<typename T>
struct id {};
typedef struct MTLTexture_ MTLTexture;
typedef struct MTLBuffer_ MTLBuffer;
typedef struct MTLSamplerState_ MTLSamplerState;
typedef struct MTLRenderCommandEncoder_ MTLRenderCommandEncoder;
typedef struct MTLPrimitiveType_ MTLPrimitiveType;
typedef struct MTLFunctionConstantValues_ MTLFunctionConstantValues;
typedef struct MTLIndexType_ MTLIndexType;
typedef struct MTLSize_ MTLSize;
typedef struct MTLRenderPipelineState_ MTLRenderPipelineState;
typedef struct MTLLibrary_ MTLLibrary;
typedef struct MTLMeshRenderPipelineDescriptor_ MTLMeshRenderPipelineDescriptor;
typedef struct MTLDevice_ MTLDevice;
typedef struct {
    uint64_t _impl;
} MTLResourceID;
// TODO: If we want to use this type in Rust, we better make sure that it comes
// from a header to keep the layout consistent?
typedef struct {
    uint32_t threadgroupsPerGrid[3];
} MTLDispatchThreadgroupsIndirectArguments;

#include <metal_irconverter_runtime/metal_irconverter_runtime.h>
#include <metal_irconverter_runtime/ir_raytracing.h>
