use std::io::{self, stdout};

use log::{error, info};
use rand::Rng;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("[{}] {}", record.level(), msg)))
        .chain(stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("Failed to configure logger");

    let secret = rand::thread_rng().gen_range(1..=128);

    info!("Wanna play a game? Guess the number...");

    loop {
        info!("You choice:");

        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to read the line: {err}");
                continue;
            }
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                error!("Failed to parse a number: {}", err);
                continue;
            }
        };

        info!("Guess is: {guess}");

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => info!("Yor guess is smaller."),
            std::cmp::Ordering::Greater => info!("Your guess is greater."),
            std::cmp::Ordering::Equal => {
                info!("Equal! Good One.");
                info!("You're rock!");
                break;
            }
        }
    }
}
