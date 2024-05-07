use std::path::Path;

fn main() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let header = crate_root.join("wrapper.h");
    let out_file = crate_root.join("../src/bindings.rs");
    let include_dir = crate_root.join("vendor");

    bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .parse_callbacks(Box::new(RenameCallback))
        .clang_args(&[
            format!("-I{}", include_dir.to_str().unwrap()).as_str(),
            "-Wno-microsoft-enum-forward-reference",
            "-fretain-comments-from-system-headers",
        ])
        .dynamic_link_require_all(true)
        .dynamic_library_name("metal_irconverter")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .bitfield_enum(".*Flags$")
        .generate_comments(true)
        .allowlist_type("IR\\w+")
        .allowlist_function("IR\\w+")
        .anon_fields_prefix("u_")
        .prepend_enum_name(false)
        // Not in the DLLs provided by Apple
        .blocklist_item("IRMetalLibSynthesizeIntersectionWrapperFunction")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

use bindgen::callbacks::{EnumVariantValue, ParseCallbacks};

#[derive(Debug)]
struct RenameCallback;
impl ParseCallbacks for RenameCallback {
    fn item_name(&self, item: &str) -> Option<String> {
        item.strip_suffix("__bindgen_ty_1")
            .map(|name| format!("{name}_u"))
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
            let enum_name = enum_name
                .strip_prefix("enum ")
                .unwrap()
                .replace("Flags", "Flag");
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
}
