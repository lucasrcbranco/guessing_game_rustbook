use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number: i8 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number! Type quit to exit the game.");

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(value) => value,
            Err(_) => continue,
        };

        if guess
            .trim()
            .to_string()
            .eq_ignore_ascii_case(&String::from("quit"))
        {
            println!("Thank you for playing");
            break;
        }

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
