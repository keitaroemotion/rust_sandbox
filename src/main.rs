use std::io;

fn main() {
    println!("guess the number");
    println!("Please input your guess.");
    // mut means mutable, since by default in Rust
    // variables are mutable.
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed {}", guess);
}
