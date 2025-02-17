use rand::Rng;
use std::io;

fn main() {
    println!("=== Chapter Two ===");
    // NOTE: we are not gonna create a new project for this
    // We are gonna write all the code in the chapter two dir
    println!("=== Guessing the number! ===");

    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret nubmer is: {secret_number}");
    println!("Please input your guess");

    //HEADER: creates a new variable with string type
    let mut guess = String::new();

    //NOTE: logic to read the user input
    //imports the standard library for input/output
    //uses the built-in read_line to read and handle errors
    // uses expect to handle panic - not very good
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_past() {
        todo!()
    }
}
