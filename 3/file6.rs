use std::fs::File;
use std::io;
use std::io::prelude::*;

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    write!(out, "answer is {}\n", 42)?;
    Ok(())
}

fn main() {
    write_out("test.txt").expect("write failed");
}
