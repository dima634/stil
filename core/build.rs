fn main() {
    cxx_build::bridge("src/ffi/mod.rs")
        .std("c++17");
}
