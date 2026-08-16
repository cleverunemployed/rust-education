use rand::{prelude::*, rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number.");

    let secret_number = rng().random_range(0..=100);

    // println!("Secret number: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Иди нахуй");

        println!("Your guess is {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
