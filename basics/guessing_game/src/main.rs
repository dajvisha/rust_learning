use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("------------------------------");
    println!("Welcome to the guessing game!");
    println!("------------------------------");

    let secret_num: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{} \n", "Too small!".red()),
            Ordering::Greater => println!("{} \n", "Too big".red()),
            Ordering::Equal => {
                println!("{} \n", "You win!".green());
                break;
            }
        }
    }
}
