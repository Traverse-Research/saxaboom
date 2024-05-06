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
        // Not in the DLLs provided by Apple
        .blocklist_item("IRMetalLibSynthesizeIntersectionWrapperFunction")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct RenameCallback;
impl ParseCallbacks for RenameCallback {
    fn item_name(&self, item: &str) -> Option<String> {
        item.strip_suffix("__bindgen_ty_1")
            .map(|name| format!("{name}_u"))
    }
}
