use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("You're playing guess the number!");

    // Generate the secret number
    let secret_number = rand::thread_rng().gen_range(1..=420);

    println!("A secret number has been generated between 0 and 420!");

    // Let users guess on a loop
    loop {
        println!("Please input your guess.");
    
        // Create new mutable variable to hold guess
        let mut guess = String::new();
    
        // Get guess input and throw error if failed
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // Convert guess to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // Check if guess is correct, gt, or lt secret_number
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("You guessed too high!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            },
            Ordering::Less => println!("You guessed too low!"),
        }
    }
}
