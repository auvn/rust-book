use log::{error, info};
use std::io::{stdin, stdout};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("[{}] {:?}", record.level(), msg)))
        .chain(stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("Failed to configure logger");

    loop {
        info!("Waiting for your input:");
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to read the line: {err}");
                continue;
            }
        }

        let first = first_word(&input);
        info!("First word is: {first}");
    }
}

fn first_word(s: &str) -> &str {
    let bb = s.as_bytes();

    for (i, &item) in bb.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
