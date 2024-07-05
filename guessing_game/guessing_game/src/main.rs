use std::io; 
use std::cmp::Ordering;
use rand::Rng;



fn main(){
    println!("guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100); // ranges from 1 to
    println!("Please input your guess");

    let mut guess = String::new();

    // Receive user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!")

        println!("You guessed: {}", guess);

        // Compares if whatever comes in guess is equal to what is in secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
}