pub mod day_8_algorithms;
pub mod day_9_structs;
pub mod day_10;

pub use std::{fs::File, io::{BufRead, BufReader, Lines}, path::PathBuf};
pub fn lines<T: ToString>(path: T) -> Lines<BufReader<File>> {
    let file = File::open(PathBuf::from(path.to_string())).unwrap();
    BufReader::new(file).lines()
}