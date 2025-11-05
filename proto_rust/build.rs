use protobuf_codegen::CodeGen;

fn main() {
    let result = CodeGen::new()
        .include("../proto")
        .input("../proto/hyprland.proto")
        .input("../proto/desktop.proto")
        .output_dir("src")
        .generate_and_compile();

    if let Err(e) = result {
        println!("Protobuf code generation error: {}", e);
    }
}
