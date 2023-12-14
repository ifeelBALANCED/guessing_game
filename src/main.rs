use guessing_game::{create_random_number, get_user_input, process};

fn main() {
    println!("Guess the number!");

    let secret_number = create_random_number();

    loop {
        let guess = match get_user_input("Please input your guess.") {
            Ok(num) => num,
            Err(_) => continue,
        };

        match process(guess, secret_number) {
            Ok(message) => {
                println!("{}", message);
                break;
            }
            Err(message) => println!("{}", message),
        }
    }
}
