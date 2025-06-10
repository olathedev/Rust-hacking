use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("my guesssing game");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("the random number is: {secret_num}");

    loop {
        println!("Please input your guess.");

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {}", user_guess);

        match user_guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Perfectttt, you win");
                break;
            }
        }
    }
}
