use std::{path::PathBuf, io::{Lines, BufReader, BufRead}, fs::File};

// Challenge: create an async crate. You'll need to use an async framework (like Tokio) since std is super limited.
pub mod forest;

// Each crates provides a public "calc_visibility" and "calc_desirabilities" function
pub mod single;
pub mod threaded;

pub fn new_forest() -> forest::Forest {
    forest_from_lines(buf_input(PathBuf::from("inputs/8.inputs.txt")))
}

fn buf_input(path: PathBuf) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).unwrap()).lines()
}

fn forest_from_lines(lines: Lines<BufReader<File>>) -> forest::Forest {
    let mut forest = forest::Forest::default();
    for line in lines.map(|f| f.unwrap()) {
        forest.0.push(Vec::new());
        for c in line.chars() {
            forest.0
            .last_mut()
            .unwrap()
            .push(forest::Tree::new(c.to_digit(10).unwrap().try_into().unwrap()));
        }
    }
    forest
}