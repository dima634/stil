fn main() {
    cxx_build::bridges([
        "src/lib.rs",
        "src/ffi.rs",
        "src/system_events.rs",
        "src/system.rs",
        "src/freedesktop/ffi.rs",
    ])
    .std("c++17");
}
