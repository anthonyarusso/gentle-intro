use std::env;

fn main() {
    let mut path = env::current_dir().expect("can't access currernt dir");
    loop {
        println!("{}", path.display());
        if ! path.pop() {
            break;
        }
    }
}
