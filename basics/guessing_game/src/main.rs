use std::io;

fn main() {
    println!("------------------------------");
    println!("Welcome to the guessing game!");
    println!("------------------------------");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("Your guess is: {}", guess);
}
