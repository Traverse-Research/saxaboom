// Bindgen produces better output when building as C and not C++.
// Undef __APPLE__ so we avoid generating bindings for dispatch/dispatch.h
#undef __APPLE__

#include <metal_irconverter/metal_irconverter.h>
