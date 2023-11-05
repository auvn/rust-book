use log::{info};
use std::{
    f64::consts::PI,
    io::{stdout},
};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, msg, record| out.finish(format_args!("[{}] {:?}", record.level(), msg)))
        .chain(stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("Failed to configure logger");

    let c1 = Circle::from_radius(10);
    let c2 = Circle::from_diameter(10);
    // let c3 = Circle::from_diameter(1);

    info!("c1 {:?} area is: {}", c1, c1.area());
    info!("c2 {:?} area is: {}", c2, c2.area());
    // info!("c3 {:?}", c3);
}

#[derive(Debug)]
struct Circle {
    radius: u64,
}

impl Circle {
    fn from_radius(radius: u64) -> Self {
        assert_ne!(radius, 0);
        Self { radius }
    }

    fn from_diameter(diameter: u64) -> Self {
        let radius = diameter / 2;
        Circle::from_radius(radius)
    }

    fn area(&self) -> f64 {
        let radius: f64 = self.radius as f64;
        radius.powi(2) * PI
    }
}
