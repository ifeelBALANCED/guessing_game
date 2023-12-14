mod input;
mod number;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = number::create_random_number();

    loop {
        let guess = match input::get_user_input("Please input your guess.") {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed the number: {}", secret_number);
                println!("You win!");
                break;
            }
        }
    }
}
