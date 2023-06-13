fn main() {
    let out_path = std::env::current_dir().unwrap();
    let header = out_path.join("wrapper.h").to_str().unwrap().to_string();
    let out_file = out_path.join("../src/bindings.rs");

    bindgen::Builder::default()
        .header(header)
        .dynamic_link_require_all(true)
        .dynamic_library_name("metal_irconverter")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {non_exhaustive: false})
        .bitfield_enum(".*Flags$")
        .blocklist_item("__darwin.*")
        .blocklist_item("__DARWIN.*")
        .blocklist_item("_DARWIN.*")
        .blocklist_item("true_")
        .blocklist_item("false_")
        .blocklist_item("__bool_true_false_are_defined")
        .blocklist_item("_opaque_pthread.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}
