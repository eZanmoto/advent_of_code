extern crate aoc;

fn main() {
    match aoc::day_6_2::run() {
        Ok(v) => match v {
            Some(n) => println!("{}", n),
            None => println!("there is no route from YOU to SAN"),
        }
        Err(e) => println!("error: {}", e),
    }
}
