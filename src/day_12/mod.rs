use super::{BufReader, File};
use std::fmt::Display;
use std::collections::{HashSet, VecDeque};

/// Key conecpts:
/// * Breadth-first search. Inspo: https://youtu.be/umszOeerdsU
/// * Modelling graph/tree data structure.
///   - http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/#modeling-graphs-in-rust-using-vector-indices
///   - http://featherweightmusings.blogspot.com/2015/04/graphs-in-rust.html
///   - https://github.com/nrc/r4cppp/blob/master/graphs/README.md
///   - In reality, we won't be saving any sort of tree structure to memory. We will just be traversing the map as if it were a tree.
/// - utf8 encodings
/// - static variables and the //unsafe block//
/// - Breaking out of nested loops with loop labels
/// Baby's first tree with Box<> smart pointers.
/// 
/// We are building a tree. We will construct this tree by starting at start and adding on each new available step as a node.
/// We will never add the same coordinate to two different nodes, because the second node would represent a longer path
///     than the first node (assuming breadth-first search). We can keep track of that by tracking the coordinates of squares added.
/// 
/// We will compare height by mapping letters to integer values by encoding to utf8.

static mut CHAR_BYTES: [u8; 1] = [0;1];

#[derive(Debug)]
pub struct ElevationMap (pub Vec<Vec<u8>>);

impl Display for ElevationMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.0 {
            for column in row {
                s.push_str(format!("{:03} ", column).as_str())
            }
            s.push('\n');
        }
        write!(f, "{s}")
    }
}

impl ElevationMap{
    pub fn new(path: &'static str) -> Self {
        let lines = super::lines(path);
        let map = lines.map(
            |line| line.unwrap().chars().map(
                |c| Self::as_int(c)
            ).collect::<Vec<u8>>()
        ).collect::<Vec<Vec<u8>>>();
        Self(map)
    }

    pub fn as_int(c: char) -> u8 {
        unsafe { 
            c.encode_utf8(&mut CHAR_BYTES); 
            CHAR_BYTES[0]
        }
    }
}

pub fn get_start_end(map: &ElevationMap) -> ((usize, usize), (usize, usize)) {
    let mut start: (usize, usize) = (0,0);
    let mut end: (usize, usize) = (0,0);
    for i in 0_usize..map.0.len() {
        for j in 0_usize..map.0[0].len() {
            if map.0[i][j] == 83 {
                println!("Start: {i}.{j}");
                start = (i, j)
            } else if map.0[i][j] == 69 {
                println!("End: {i}.{j}");
                end = (i, j);
            }
        }
    }
    (start, end)
}

pub fn neighbors(map: &ElevationMap, coordinate: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    let mut valid_neighbors: [Option<(usize, usize)>; 4] = [None; 4];      // initialize a coordinates buffer.

    let &(row, column) = coordinate;
    let mut elevation = map.0[row][column];
    if elevation == 69 {
        elevation = ElevationMap::as_int('z');
    } else if elevation == 83 {
        elevation = ElevationMap::as_int('a');
    }

    if row > 0 {
        if test_cell(map.0[row - 1][column], elevation) {
            valid_neighbors[0] = Some((row - 1, column));
        }
    }
    if row < map.0.len() - 1 {
        if test_cell(map.0[row + 1][column], elevation) {
            valid_neighbors[1] = Some((row + 1, column));
        }
    }
    if column > 0 {
        if test_cell(map.0[row][column - 1], elevation) {
            valid_neighbors[2] = Some((row, column - 1));
        }
    }
    if column < map.0[0].len() - 1 {
        if test_cell(map.0[row][column + 1], elevation) {
            valid_neighbors[3] = Some((row, column + 1));
        }
    }

    valid_neighbors
}

pub fn rev_neighbors(map: &ElevationMap, coordinate: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
    let mut valid_neighbors: [Option<(usize, usize)>; 4] = [None; 4];      // initialize a coordinates buffer.

    let &(row, column) = coordinate;
    let mut elevation = map.0[row][column];
    if elevation == 69 {
        elevation = ElevationMap::as_int('z');
    } else if elevation == 83 {
        elevation = ElevationMap::as_int('a');
    }

    if row > 0 {
        if rev_test_cell(map.0[row - 1][column], elevation) {
            valid_neighbors[0] = Some((row - 1, column));
        }
    }
    if row < map.0.len() - 1 {
        if rev_test_cell(map.0[row + 1][column], elevation) {
            valid_neighbors[1] = Some((row + 1, column));
        }
    }
    if column > 0 {
        if rev_test_cell(map.0[row][column - 1], elevation) {
            valid_neighbors[2] = Some((row, column - 1));
        }
    }
    if column < map.0[0].len() - 1 {
        if rev_test_cell(map.0[row][column + 1], elevation) {
            valid_neighbors[3] = Some((row, column + 1));
        }
    }

    valid_neighbors
}


fn test_cell(neighbor: u8, current: u8) -> bool {
    neighbor <= current + 1
}

fn rev_test_cell(neighbor: u8, current: u8) -> bool {
    current <= neighbor + 1
}

#[cfg(test)]
mod day_12_tests {
    use super::*;
    #[test]
    fn test_char_parser() {
        assert_eq!(ElevationMap::as_int('a'), 97);
        assert_eq!(ElevationMap::as_int('z'), 122);
    }

    #[test]
    fn test_new_map() {
        let map = ElevationMap::new("inputs/12.inputs.txt");
        println!("{map}");
    }

}