// Bindgen produces better output when building as C and not C++.
typedef struct IRError IRError;
typedef struct IRRootSignature IRRootSignature;
typedef struct IRObject IRObject;
typedef struct IRCompiler IRCompiler;
typedef struct IRMetalLibBinary IRMetalLibBinary;
typedef struct IRShaderReflection IRShaderReflection;
typedef enum IRReflectionVersion IRReflectionVersion;

// Undef __APPLE__ so we avoid generating bindings for dispatch/dispatch.h
#undef __APPLE__

#include <metal_irconverter/metal_irconverter.h>
