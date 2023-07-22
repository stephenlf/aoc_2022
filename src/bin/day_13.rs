
use advent_of_code::day_13::*;
use advent_of_code::lines;

fn main() {
    let mut lines = lines("inputs/13.inputs.txt");

    let mut i: u32 = 0;
    loop {
        i += 1;
        let left_list = List::new(lines.next().unwrap().unwrap());
        let right_list = List::new(lines.next().unwrap().unwrap());

        if lines.next().is_none() {
            break;
        }
    }
}