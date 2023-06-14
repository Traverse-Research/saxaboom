use bindgen::callbacks::ParseCallbacks;

#[derive(Debug)]
struct RenameCallback;
impl ParseCallbacks for RenameCallback {
    fn item_name(&self, item: &str) -> Option<String> {
        match item {
            "IRError" => Some("IRErrorOpaque".to_string()),
            "IRRootSignature" => Some("IRRootSignatureOpaque".to_string()),
            "IRObject" => Some("IRObjectOpaque".to_string()),
            "IRCompiler" => Some("IRCompilerOpaque".to_string()),
            "IRMetalLibBinary" => Some("IRMetalLibBinaryOpaque".to_string()),
            "IRShaderReflection" => Some("IRShaderReflectionOpaque".to_string()),
            v if item.contains("_bindgen_ty_1") => {
                Some(v.replace("__bindgen_ty_1", "_u").to_string())
            }
            v if item.contains("_bindgen_ty_") => {
                Some(v.replace("__bindgen_ty_", "_u_").to_string())
            }
            _ => None,
        }
    }
}

fn main() {
    let out_path = std::env::current_dir().unwrap();
    let header = out_path.join("wrapper.h").to_str().unwrap().to_string();
    let types_file = out_path.join("../src/types.rs");
    let dll_file = out_path.join("../src/bindings.rs");

    bindgen::Builder::default()
        .header(&header)
        .parse_callbacks(Box::new(RenameCallback))
        .clang_args(&["-I./vendor/", "-Wno-microsoft-enum-forward-reference"])
        .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]")
        // .dynamic_link_require_all(true)
        // .dynamic_library_name("metal_irconverter")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .bitfield_enum(".*Flags$")
        .blocklist_item("__darwin.*")
        .blocklist_item("__DARWIN.*")
        .blocklist_item("_DARWIN.*")
        .blocklist_item("true_")
        .blocklist_item("false_")
        .blocklist_item("__bool_true_false_are_defined")
        .blocklist_item("_opaque_pthread.*")
        .blocklist_item("__security_init_cookie")
        .blocklist_item("__security_check_cookie")
        .blocklist_item("__security_cookie")
        .blocklist_item("__va_start")
        .blocklist_item("__report_gsfailure")
        .anon_fields_prefix("u_")
        .allowlist_type("IR.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(types_file)
        .expect("Couldn't write bindings!");

    bindgen::Builder::default()
        .header(&header)
        .parse_callbacks(Box::new(RenameCallback))
        .clang_args(&["-I./vendor/", "-Wno-microsoft-enum-forward-reference"])
        .raw_line("#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]")
        .raw_line("use crate::types::*;")
        .dynamic_link_require_all(true)
        .dynamic_library_name("MetalIrConverter")
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .bitfield_enum(".*Flags$")
        .blocklist_item("__darwin.*")
        .blocklist_item("__DARWIN.*")
        .blocklist_item("_DARWIN.*")
        .blocklist_item("true_")
        .blocklist_item("false_")
        .blocklist_item("__bool_true_false_are_defined")
        .blocklist_item("_opaque_pthread.*")
        .blocklist_item("__security_init_cookie")
        .blocklist_item("__security_check_cookie")
        .blocklist_item("__security_cookie")
        .blocklist_item("__va_start")
        .blocklist_item("__report_gsfailure")
        // Not in the DLLs provided by Apple
        .blocklist_item("IRMetalLibSynthesizeIntersectionWrapperFunction")
        .blocklist_type(".*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(dll_file)
        .expect("Couldn't write bindings!");
}
