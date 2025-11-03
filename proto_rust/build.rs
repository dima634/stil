use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .protoc()
        .includes(&["../proto"])
        .input("../proto/hyprland.proto")
        .out_dir("src/proto")
        .run_from_script();
}
