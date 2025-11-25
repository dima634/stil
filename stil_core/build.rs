fn main() {
    cxx_build::bridges([
        "src/lib.rs",
        "src/system_events.rs",
        "src/system.rs",
        "src/application.rs",
        "src/ffi/mod.rs",
        "src/ffi/hyprland.rs",
        "src/freedesktop/ffi.rs",
    ])
    .std("c++17");
}
