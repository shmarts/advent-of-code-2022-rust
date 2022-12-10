use std::fs;

use day_01::process_part1;

fn main() {
    let input = fs::read_to_string("./day-01/input.txt").unwrap();
    println!("{}", process_part1(&input));
}
