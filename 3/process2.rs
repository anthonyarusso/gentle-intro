use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("-V")
        .output()
        .expect("Failed to launch rustc.");

    if output.status.success() {
        println!("ok!");
    }

    println!("len stdout {} stderr {}", output.stdout.len(), output.stderr.len());
}
