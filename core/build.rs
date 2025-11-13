fn main() {
    cxx_build::bridges(["src/ffi/mod.rs", "src/ffi/system_events.rs"])
        .std("c++17");
}
