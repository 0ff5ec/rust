use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1,101);

    println!("Secret number is {}", secret);

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line!");
    
    println!("You guessed: {}", guess);
}
