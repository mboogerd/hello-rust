use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    // let is used for variables, by default immutable
    // mut is used to explicitly mark them as mutable
    // [Type]::[function] marks an `associated` (static) function
    let mut guess = String::new();

    // & indicates that an argument is a `reference` (immutable by default)
    // reference variables can also be marked mutable
    io::stdin().read_line(&mut guess)
    // An instance of io::Result has an expect method that you can call.
    // If this instance of io::Result is an Err value, expect will cause
    // the program to crash and display the message that you passed as an
    // argument to expect
        .expect("Failed to read line");

    // One can submit placeholders in strings, and supply the values
    // separately
    println!("You guessed: {}", guess);
}