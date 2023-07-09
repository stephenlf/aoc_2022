#![allow(unused, unused_imports, unused_variables)]

pub use std::{path::PathBuf, io::{Lines, BufReader, BufRead}, fs::File};

pub mod shipping;

pub fn read_to_lines(path: PathBuf) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    lines
}

// ! Read file input to Port struct. 
// _ Parse instructions.
// _ Run program

// Instructions:
//    move x from y to z.
//  where x = number of crates
//        y = origin dock
//        z = destination dock

pub fn parse_instruction(line: String) -> (u32, usize, usize) {

    let tokens = line
        .split_ascii_whitespace()
        .map(|s: &str| s.to_owned())
        .collect::<Vec<String>>();

    let num_crates: u32 = tokens[1].parse::<u32>().expect("Issue parsing num_crates");
    let origin: usize = tokens[3].parse::<usize>().expect("Issue parsing orign");
    let dest: usize = tokens[5].parse::<usize>().expect("Issue parsing dest");

    (num_crates, origin, dest)

}



#[cfg(test)]
mod _day_5_tests {
    use super::parse_instruction;

    #[test]
    fn test_parse_instructions() {
        let line = "move 6 from 1 to 7".to_owned();
        let (x, y, z) = parse_instruction(line);
        assert_eq!(x, 6);
        assert_eq!(y, 1);
        assert_eq!(z, 7);
    }
}