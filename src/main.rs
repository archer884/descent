use clap::Parser;

/// Standard three degree descent angle
const BETA: f64 = 87.0 * std::f64::consts::PI / 180.0;

/// Length of nautical mile in feet
const FEET_PER_NM: f64 = 6076.1155;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Calculates top of descent for a given altitude, target altitude, and
/// flight path angle.
struct Args {
    /// initial altitude
    initial: f64,

    /// target altitude
    target: f64,

    /// desired descent angle
    #[arg(short = 'a', long = "angle")]
    descent_angle: Option<f64>,

    /// Provide groundspeed in knots and the program will print the approximate
    /// descent rate required in feet per minute. You can also use true airspeed,
    /// except of course head and tailwinds will mess it up. Indicated airspeed
    /// will NOT work.
    #[arg(short = 's', long = "speed")]
    groundspeed: Option<f64>,
}

impl Args {
    fn run(&self) {
        let distance = self.solve_for_delta();
        println!("{distance:.02} NM");

        if let Some(groundspeed) = self.groundspeed {
            let time = distance / groundspeed;
            let delta = (self.initial - self.target).abs();
            let hundreds_of_feet_per_minute = (delta / time / 60.0 / 100.0).ceil();
            println!("{:.0} fpm", hundreds_of_feet_per_minute * 100.0);
        }
    }

    // Formula for the opposite leg of a right triangle:
    // b = a * tan(Beta)
    fn solve_for_delta(&self) -> f64 {
        let delta = (self.initial - self.target).abs();
        let horizontal_distance_in_feet = delta * self.climb_descend_angle().tan();
        horizontal_distance_in_feet / FEET_PER_NM
    }

    fn climb_descend_angle(&self) -> f64 {
        self.descent_angle
            .map(convert_descent_angle)
            .unwrap_or(BETA)
    }
}

fn main() {
    Args::parse().run();
}

fn convert_descent_angle(degrees: f64) -> f64 {
    (90.0 - degrees % 90.0).to_radians()
}
