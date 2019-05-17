// This handle input/output.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("The secret number is: {}", secret_number);

        print!("Please input your guess:");

        // variables in Rust are immutable by default.
        // (mut) as in mutable.
        let mut guess = String::new(); // String::new means new is a function of String.

        // We are using std::io in here.
        // :: -> Get access to the instance Stdin, basically a class type than handle input.
        io::stdin().read_line(&mut guess) // .read_line() is a method. passing a arg that store any things that the user typed as string. Remeber that we are referencing mut but not guess. (&mut but not &guess).
            // .ok() Result successful
            // .err() Result unsuccessful
            .expect("Failed to read line"); // Handling potential failure with the Result type.

        // Although we declared guess before. But rust allowed us to shadowing.
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Although rustup using u32 but that is inefficient. u32 is way bigger than what we needed.

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
