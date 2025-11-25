use std::io::Read;

fn main() {
    stil_core::init();
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
