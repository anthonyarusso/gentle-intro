fn main() {
    let number = 15;

    let size = match number {
        0..=3 => "small",
        4..=6 => "medium",
        7..=10 => "large",
        _ => "extra large"
    };

    println!("number is {}, size is {}", number, size);
}
