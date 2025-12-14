use std::process::Command;

fn main() {
    let input = "src/styles.scss";
    let output = "src/styles.css";

    println!("cargo:rerun-if-changed={}", input);

    let status = Command::new("sassc")
        .args([input, output])
        .status()
        .expect("failed to run sassc");

    if !status.success() {
        panic!("sass compilation failed");
    }
}
