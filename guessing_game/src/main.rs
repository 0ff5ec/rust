use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1,101);

    println!("Secret number is {}", secret);

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line!");

    let guess: u32 = guess.trim().parse()
        .expect("Please enter a number!");
    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    }
}
