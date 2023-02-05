use rand::Rng;
use std::io;

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

    println!("Your guess is: {}", guess);
}
