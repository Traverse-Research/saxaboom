# 🤘 Saxaboom

[![Actions Status](https://github.com/Traverse-Research/saxaboom/actions/workflows/ci.yml/badge.svg)](https://github.com/Traverse-Research/saxaboom/actions)
[![Latest version](https://img.shields.io/crates/v/saxaboom.svg?logo=rust)][`saxaboom`]
[![Documentation](https://img.shields.io/docsrs/saxaboom/latest?logo=docs.rs)](https://docs.rs/saxaboom)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](./CODE_OF_CONDUCT.md)

[![Banner](banner.png)](https://traverseresearch.nl)

[`saxaboom`] is a small helper library around [Metal shader converter] to create metal shader libraries from `DXIL` files (HLSL source code).  See also [`saxaboom-runtime`] which provides the runtime structures and interop with the [`metal`] crate needed to make use of the resulting `metallib` shaders.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
saxaboom = "0.2.0"
```

Example to compile `DXIL` to `metallib`:

```rust,no_run
use saxaboom::{ffi, MetalIrConverter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the library
    let metal_irconverter =
        MetalIrConverter::new(libloading::library_filename("metalirconverter")).unwrap();
    // Create an instance of IRCompiler
    let mut compiler = metal_irconverter.create_compiler();
    // Create an object containing DXIL bytes, replace &[0u8] with your DXIL data
    let dxil = metal_irconverter.create_object_from_dxil(&[0u8]);

    // See `IRCompiler` docs for possible state that can be set on the compiler before compiling
    // DXIL source, such as a global root signatures and various raytracing parameters.

    // Compile the `dxil` data blob with entrypoint `main` into mtllib
    let mtllib = compiler.alloc_compile_and_link(c"main", &dxil)?;

    let reflection = mtllib.reflection();
    let mtl_binary = mtllib
        .metal_lib_binary()
        .expect("Compiled object should contain a `metallib`");
    let bytecode = mtl_binary.byte_code();

    Ok(())
}
```

For using the loaded `metallib` shaders at runtime most effectively, consult [`saxaboom-runtime`].

[Metal shader converter]: https://developer.apple.com/metal/shader-converter/
[`saxaboom`]: https://crates.io/crates/saxaboom
[`saxaboom-runtime`]: https://crates.io/crates/saxaboom-runtime
[`metal`]: https://crates.io/crates/metal
