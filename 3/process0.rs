use std::process::Command;

fn main() {
    let status = Command::new("rustc")
        .arg("-V")
        .status()
        .expect("Failed to launch rustc");

    println!("cool {} code {}", status.success(), status.code().unwrap());
}
