use log::info;
use std::io::stdout;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("[{}] {:?}", record.level(), msg)))
        .chain(stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("Failed to configure logger");

    let plant = Asparagus {};
    info!("{:?}", plant);
}
