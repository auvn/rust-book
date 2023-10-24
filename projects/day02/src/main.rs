use std::io;

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=128);

    println!("Guess the number");

    loop {
        println!("Your try:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It should be a number.");
                continue;
            }
        };

        println!("Guess is: {guess}");

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Small."),
            std::cmp::Ordering::Greater => println!("Greater than expected."),
            std::cmp::Ordering::Equal => {
                println!("Equal! Good One.");
                println!("You're rock!");
                break;
            }
        }
    }
}
