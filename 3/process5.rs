use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("rustc")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to launch rustc.");

    let res = child.wait();
    println!("res {:?}", res);
}
