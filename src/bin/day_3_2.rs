extern crate aoc;

fn main() {
    match aoc::day_3_2::run() {
        Ok(closest) => match closest {
            Some(v) => println!("{}", v),
            None => println!("error: paths do not overlap"),
        },
        Err(e) => println!("error: {}", e),
    }
}
