use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    initial: f64,
    target: f64,

    /// desired descent angle
    descent_angle: Option<f64>,
}

impl Args {
    fn run(&self) {
        let distance = self.solve_for_delta();
        println!("{distance:.02} NM");
    }

    // Formula for the opposite leg of a right triangle:
    // b = a * tan(Beta)
    fn solve_for_delta(&self) -> f64 {
        // Standard three degree descent angle
        const BETA: f64 = 87.0 * std::f64::consts::PI / 180.0;

        // Length of nautical mile in feet
        const FEET_PER_NM: f64 = 6076.1155;

        let climb_descent_angle = self
            .descent_angle
            .map(convert_descent_angle)
            .unwrap_or(BETA);

        let delta = (self.initial - self.target).abs();
        let horizontal_distance_in_feet = delta * climb_descent_angle.tan();

        horizontal_distance_in_feet / FEET_PER_NM
    }
}

fn main() {
    Args::parse().run();
}

fn convert_descent_angle(degrees: f64) -> f64 {
    (90.0 - degrees % 90.0).to_radians()
}
