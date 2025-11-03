use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .protoc()
        .includes(&["../stil_proto"])
        .input("../stil_proto/hyprland.proto")
        .out_dir("src/proto")
        .run_from_script();
}
