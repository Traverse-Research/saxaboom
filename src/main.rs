use std::path::Path;

use saxaboom::compile_dxil_to_metallib_from_path;

fn main() {
    let shaders_to_test = ["test/output.dxil"];
    for shader_path in shaders_to_test {
        print!("Testing shader \"{shader_path}\": ");
        match compile_dxil_to_metallib_from_path(&Path::new(shader_path)) {
            Ok(_) => println!("\tOk."),
            Err(e) => println!("Error: {e:?}"),
        }
    }
}
