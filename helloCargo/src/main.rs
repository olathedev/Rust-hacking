use std::io;

fn main() {
    println!("my guesssing game");
    println!("Please input your guess.");
    let mut user_guess = String::new();

    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

    println!("you guessed {}", user_guess)

}
