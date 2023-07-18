///*
/// State machines with enums
/// Hash set 
/// TryFrom trait
/// Error trait
/// Lib crate modules (extension of previous puzzle)
/// 
/// We learn that this state machine didn't scale at all
/// 
/// WHAT A HEADACHE
/// */

/* 
Save absolute position of tail and relative position of head
match relative position + move
React with updated relative position and absolute position
 */

use std::{path::PathBuf, io::{Lines, BufReader, BufRead}, fs::File};

pub use advent_of_code::day_9_structs::*;
use advent_of_code::day_9_structs::part_2::*;

fn read_lines(path: PathBuf) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn main() {
    let lines = read_lines(PathBuf::from("inputs/9.inputs.txt"));

    let mut state = StateMachine::new();
    for line in lines {
        state.execute_line(line.unwrap());

    }
    println!("part 1: {:?}",state.visited.len());

    // PART 2

    let lines = read_lines(PathBuf::from("inputs/9.inputs.txt"));
    let mut rope = Rope::new(10);
    for line in lines {
        rope.step(line.unwrap());
    }
    println!("part 2: {}", rope.size());

}

#[cfg(test)]
mod test_9 {
    use super::*;
    #[test]
    fn bro() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let mut state = StateMachine::new();
        let lines = input.lines();
        for line in lines {
            state.execute_line(line.to_owned());
        }
        println!("{:?}\n{}",&state.visited,&state.visited.len());
        assert_eq!(13, state.visited.len());
    }

    #[test]
    fn gro() {
        let input = "U 1
R 1
U 1
R 1";

        let mut state = StateMachine::new();
        let lines = input.lines();
        for line in lines {
            state.execute_line(line.to_owned());
        }
        println!("{:?}\n{}",&state.visited,&state.visited.len());
        assert_eq!(2, state.visited.len());
    }
}