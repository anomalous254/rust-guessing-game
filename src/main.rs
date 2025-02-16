use std::io;
use rand;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game");

    let secret_number: u32 = rand::random_range(1..=100);

    loop {
        println!("Please enter a number (1-100) or 'q' to quit:");

        let mut guess : String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess : &str = guess.trim();

        // Check if the user wants to quit
        if guess.eq_ignore_ascii_case("q") {
            println!("Goodbye!");
            break;
        }

        // Try parsing the input as a number
        let guess: u32 = match guess.parse() {
            Ok(num) if num >= 1 && num <= 100 => num,
            Ok(_) => {
                println!("Please enter a valid number between 1 and 100.");
                continue;
            }
            Err(_) => {
                println!("Invalid input! Please enter a number or 'q' to quit.");
                continue;
            }
        };

        println!("Your Guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;
            }
        }
    }
}
