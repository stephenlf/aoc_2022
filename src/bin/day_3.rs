#![allow(non_snake_case, dead_code, unused)]

use std::collections::HashSet;
use std::collections::hash_set::Intersection;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn file_to_lines(path: PathBuf) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn line_to_sets(s: String) -> (HashSet<char>, HashSet<char>) {
    let mut left_side: HashSet<char> = HashSet::new();
    let mut right_side: HashSet<char> = HashSet::new();
    let middle = s.len() / 2;

    for (i, c) in s.chars().enumerate() {
        if i < middle {
            left_side.insert(c);
        }
        else {
            right_side.insert(c);
        }
    }

    (left_side, right_side)
}

fn group_to_sets(a: String, b: String, c: String) -> [HashSet<char>; 3] {
    let a_set: HashSet<char> = a.chars().collect();
    let b_set: HashSet<char> = b.chars().collect();
    let c_set: HashSet<char> = c.chars().collect();

    [a_set, b_set, c_set]
}

fn priority(c: &char) -> u32 {
    let c: u32 = (*c).into();
    match c {
        65..=90 => c-65+27,
        97..=122 => c-96,
        _ => 0,
    }
}

fn collect_three(lines: &mut Lines<BufReader<File>>) -> Option<[String; 3]> {
    let [mut a, mut b, mut c] = [String::new(), String::new(), String::new()];
    a = lines.next()?.unwrap();
    b = lines.next()?.unwrap();
    c = lines.next()?.unwrap();
    Some([a, b, c])
}

fn main() {
    let lines = file_to_lines(PathBuf::from("inputs/3.input.txt"));
    let mut total: u32 = 0;
    for (left, right) in lines.map(|l| line_to_sets(l.unwrap())) {
        total += priority(left.intersection(&right).next().unwrap());
    }
    println!("Challenge 1: {}",total);
    
    let mut lines = file_to_lines(PathBuf::from("inputs/3.input.txt"));
    let mut total_2: u32 = 0;

    loop {
        match collect_three(&mut lines) {
            Some([a, b, c]) => {
                let [a_set, b_set, c_set] = group_to_sets(a, b, c);
                let intersection_a_b: HashSet<char> = a_set
                    .intersection(&b_set)
                    .map(|c| c.to_owned())
                    .collect();
                let intersection_a_b_c = intersection_a_b
                    .intersection(&c_set)
                    .map(|c| c.to_owned())
                    .next()
                    .unwrap();
                total_2 += priority(&intersection_a_b_c);
            }
            None => {
                break;
            }
        }
    }
    println!("{}",total_2);


}
#[cfg(test)]
mod test_3 {
    use std::{collections::HashSet, path::PathBuf};

    use crate::{line_to_sets, group_to_sets, file_to_lines, collect_three};

    use super::priority;

    #[test]
    fn test_priority() {
        assert_eq!(priority(&'a'), 1);
        assert_eq!(priority(&'z'), 26);
        assert_eq!(priority(&'A'), 27);
        assert_eq!(priority(&'Z'), 52);
    }

    #[test]
    fn test_line_to_vecs() {
        let line = String::from("heyHEY");
        let (left, right) = line_to_sets(line);
        assert_eq!(left, vec!['h','e','y'].into_iter().collect());
        assert_eq!(right, vec!['H','E','Y'].into_iter().collect());

    }

    #[test]
    fn test_union() {
        let (left, right) = line_to_sets(String::from("axabbx"));
        let intersection = left.intersection(&right).next().unwrap();
        assert_eq!(intersection, &'x');
    }

    #[test]
    fn test_group_to_sets() {
        let a = String::from("cCdDx");
        let b = String::from("xeEfF");
        let c = String::from("aAxbB");
        let [a_set, b_set, c_set] = group_to_sets(a, b, c);
        let intersection_a_b: HashSet<char> = a_set
            .intersection(&b_set)
            .map(|c| c.to_owned())
            .collect();
        let intersection_a_b_c = intersection_a_b
            .intersection(&c_set)
            .map(|c| c.to_owned())
            .next()
            .unwrap();
        assert_eq!(intersection_a_b_c, 'x');
    }

    #[test] 
    fn test_collect_three() {
        let mut lines = file_to_lines(PathBuf::from("inputs/3.input.txt"));
        println!("{:?}", collect_three(&mut lines));
        println!("{:?}", collect_three(&mut lines));
    }
}