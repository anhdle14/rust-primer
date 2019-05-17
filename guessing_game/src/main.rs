// This handle input/output.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // variables in Rust are immutable by default.
    // (mut) as in mutable.
    let mut guess = String::new(); // String::new means new is a function of String.

    // We are using std::io in here.
    // :: -> Get access to the instance Stdin, basically a class type than handle input.
    io::stdin().read_line(&mut guess) // .read_line() is a method. passing a arg that store any things that the user typed as string. Remeber that we are referencing mut but not guess. (&mut but not &guess).
        // .ok() Result successful
        // .err() Result unsuccessful
        .expect("Failed to read line"); // Handling potential failure with the Result type.


    println!("You guessed: {}", guess)
}
