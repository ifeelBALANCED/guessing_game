use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn process(guess: u32, secret_number: u32) -> Result<&'static str, &'static str> {
    match guess.cmp(&secret_number) {
        Ordering::Less => Err("Too small!"),
        Ordering::Greater => Err("Too big!"),
        Ordering::Equal => Ok("You win!"),
    }
}

pub fn get_user_input(prompt: &str) -> Result<u32, &'static str> {
    println!("{}", prompt);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => {
            println!("Please enter a valid number.");
            Err("Invalid input")
        }
    }
}

pub fn create_random_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

pub fn mocked_game_loop(guess: u32, secret_number: u32) -> &'static str {
    match process(guess, secret_number) {
        Ok(message) => message,
        Err(message) => message,
    }
}

pub fn mocked_user_input(input: &str) -> Result<u32, &'static str> {
    match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_number_within_range() {
        let num = create_random_number();
        assert!(
            num >= 1 && num <= 100,
            "Random number should be between 1 and 100"
        );
    }

    #[test]
    fn test_input_functionality() {
        assert_eq!(mocked_user_input("42"), Ok(42));
    }

    #[test]
    fn test_main_logic() {
        assert_eq!(mocked_game_loop(32, 32), "You win!");
        assert_eq!(mocked_game_loop(10, 32), "Too small!");
        assert_eq!(mocked_game_loop(55, 32), "Too big!");
    }
}
