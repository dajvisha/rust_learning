use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("------------------------------");
    println!("Welcome to the guessing game!");
    println!("------------------------------");

    let secret_num: i32 = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("Your guess is: {}", guess);

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big"),
    }
}
