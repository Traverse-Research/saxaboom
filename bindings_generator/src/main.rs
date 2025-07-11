use std::path::Path;

fn main() {
    compiler_bindings();
    runtime_bindings();
}

fn compiler_bindings() {
    let msrv = bindgen::RustTarget::stable(81, 0).unwrap();

    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let header = crate_root.join("wrapper.h");
    let out_file = crate_root.join("../src/bindings.rs");
    let include_dir = crate_root.join("vendor");

    bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .rust_target(msrv)
        .parse_callbacks(Box::new(RenameCallback))
        .clang_args(&["-I", include_dir.to_str().unwrap()])
        .dynamic_link_require_all(true)
        .dynamic_library_name("metal_irconverter")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .bitfield_enum(".*Flags$")
        .generate_comments(true)
        .allowlist_item("IR\\w+")
        .anon_fields_prefix("u_")
        .prepend_enum_name(false)
        // Not in the DLLs provided by Apple
        .blocklist_item("IRMetalLibSynthesizeIntersectionWrapperFunction")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

fn runtime_bindings() {
    let msrv = bindgen::RustTarget::stable(81, 0).unwrap();

    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let header = crate_root.join("runtime_wrapper.h");
    let out_file = crate_root.join("../runtime/src/bindings.rs");
    let include_dir = crate_root.join("vendor");

    bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .rust_target(msrv)
        .parse_callbacks(Box::new(RenameCallback))
        .clang_args(&["-I", include_dir.to_str().unwrap(), "-xc++"])
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .derive_default(true)
        .generate_comments(true)
        .allowlist_recursively(false)
        // Only allowlist types and variables (constants), skip all function declarations
        .allowlist_var("kIR\\w+")
        .allowlist_type("IR\\w+")
        // Block a few types that contain obj-C types but are only used in the "inline" functions
        // that we're skipping, to simplify our bindings.
        .blocklist_type("IRGeometryEmulationPipelineDescriptor")
        .blocklist_type("IRGeometryTessellationEmulationPipelineDescriptor")
        .blocklist_type("IRBufferView")
        // Specific (POD) types that we include explicitly
        .allowlist_type("MTLDispatchThreadgroupsIndirectArguments")
        .allowlist_type("dispatchthreadgroupsindirectargs_t")
        .allowlist_type("uint")
        .allowlist_type("resourceid_t")
        .anon_fields_prefix("u_")
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

use bindgen::callbacks::{EnumVariantValue, ItemInfo, ItemKind, ParseCallbacks};

#[derive(Debug)]
struct RenameCallback;
impl ParseCallbacks for RenameCallback {
    fn item_name(&self, item: ItemInfo<'_>) -> Option<String> {
        if let ItemKind::Type = item.kind {
            item.name
                .strip_suffix("__bindgen_ty_1")
                .map(|name| format!("{name}_u"))
        } else {
            None
        }
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        // Remove the enum name from the variant name:
        // `IRShaderStage::IRShaderStageVertex` -> `IRShaderStage::Vertex`
        if let Some(enum_name) = enum_name {
            let mut enum_name = enum_name
                .strip_prefix("enum ")
                .unwrap_or(enum_name)
                .replace("Flags", "Flag");

            // For this specific enum, strip off the `Primitive` suffix (like the `Flags` suffix above)
            if enum_name == "IRRuntimeTessellatorOutputPrimitive" {
                enum_name = String::from("IRRuntimeTessellatorOutput");
            }

            // For these two specific enums, strip off the `Mode` suffix which is not repeated in
            // their variants.  Note that a generic `strip_suffix()` solution won't work because
            // enums like `IRTextureAddressMode` _do_ share the `Mode` suffix in their variants.
            if enum_name == "IRRayGenerationCompilationMode" {
                enum_name = String::from("IRRayGenerationCompilation");
            }
            if enum_name == "IRIntersectionFunctionCompilationMode" {
                enum_name = String::from("IRIntersectionFunctionCompilation");
            }

            let new_name = original_variant_name
                .strip_prefix(&enum_name)
                .unwrap_or(original_variant_name);
            let mut new_name = new_name.strip_prefix('_').unwrap_or(new_name).to_string();
            if new_name.chars().next().unwrap().is_ascii_digit() {
                new_name.insert(0, '_');
            }
            return Some(new_name);
        }
        None
    }

    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        if name.starts_with("IRIntrinsicMask") {
            // Match the argument type passed to IRCompilerSetRayTracingPipelineArguments
            Some(bindgen::callbacks::IntKind::U64)
        } else {
            None
        }
    }
}
