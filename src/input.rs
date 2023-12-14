use std::io;

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
