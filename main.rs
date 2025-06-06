use rand::Rng; // random number generator
use std::cmp::Ordering;
use std::io; // input output from standard lib for comparing numbers
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                println!("Please enter a valid number");
                continue;
            }
        };


        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{ println!("You Won the game!");
            break;
        }
        }
    }
}
