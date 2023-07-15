#![allow(unused)]

/// Key topics covered:
/// * Bool algebra syntax

use std::{fs::File, path::PathBuf};
use std::io::{BufRead, BufReader, Lines};

/*---------------------BUSINESS-----------------------*/
// Approach: for each pair of elves, one fully encloses the other
// if min(elf_a) > min(elf_b) and max(elf_a) > max(elf_b) 
// [or vice versa]

// Model data
struct Elf {
    assignment: (u32, u32)
}

// Comparator
impl Elf {
    fn contains(&self, e: &Elf) -> Contains {
        let (s_min, s_max) = self.assignment;
        let (e_min, e_max) = e.assignment;

        if s_min <= e_min && s_max >= e_max {
            Contains::Left
        }
        else if s_min >= e_min && s_max <= e_max {
            Contains::Right
        }
        else {
            Contains::None
        }
    }

    fn partial_contains(&self, e: &Elf) -> bool {
        let (s_min, s_max) = self.assignment;
        let (e_min, e_max) = e.assignment;

        (s_min == e_min) 
            || (s_max == e_min)
            || (s_min == e_max)
            || (s_max == e_max)
            || (s_min > e_min && s_min < e_max) // s_min within e_range
            || (s_max > e_min && s_max < e_max) // s_max within e_range
            || (e_min > s_min && e_min < s_max) // e_max within s_range
            || (e_max > s_min && e_max < s_max) // e_max within s_range

    }
}

// Result of comparator function
#[derive(PartialEq, Eq, Debug)]
enum Contains {
    Left,
    Right,
    None,
}

fn parse_line(line: String) -> (Elf, Elf) {
    let pairs: Vec<&str> = line.split(',').collect();
    let left: Vec<&str> = pairs[0].split('-').collect();
    let right: Vec<&str> = pairs[1].split('-').collect();
    // left [or right] is of form "vec![123,456]"
    let left_elf = Elf {
        assignment: 
            (left[0].parse::<u32>().unwrap(),
            left[1].parse::<u32>().unwrap())
    };
    let right_elf = Elf {
        assignment: 
            (right[0].parse::<u32>().unwrap(),
            right[1].parse::<u32>().unwrap())
    };
    (left_elf, right_elf)
}


/*---------------------UTILITY-----------------------*/
fn read_file(path: PathBuf) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    lines
}


/*---------------------MAIN-----------------------*/
fn main() {
    let lines = read_file(PathBuf::from("inputs/4.input.txt"));
    
    let mut total: u32 = 0;
    
    
    for line in lines {
        let (left_elf, right_elf) = parse_line(line.unwrap());
        match left_elf.contains(&right_elf) {
            Contains::Left => {total += 1}
            Contains::Right => {total += 1}
            Contains::None => {total += 0}
        }
    }
    
    let lines = read_file(PathBuf::from("inputs/4.input.txt"));
    let mut total_partial: u32 = 0;

    for line in lines {
        let (left_elf, right_elf) = parse_line(line.unwrap());
        
        match left_elf.partial_contains(&right_elf) {
            true => {total_partial += 1}
            false => {total_partial += 0}
        }
    }

    println!("Challenge 1 (number of intersections): {}", total);
    println!("Challenge 2 (number of partial intersections): {}", total_partial);
}


/*----------------------TESTS----------------------*/
#[cfg(test)]
mod test_4 {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (left_elf, right_elf) = 
            parse_line("2-4,6-8".to_owned());
        assert_eq!(left_elf.contains(&right_elf), Contains::None);
        
        let (left_elf, right_elf) = 
            parse_line("1-99,2-3".to_owned());
        assert_eq!(left_elf.contains(&right_elf), Contains::Left);

        let (left_elf, right_elf) = 
            parse_line("1-3,2-4".to_owned());
        assert_eq!(left_elf.contains(&right_elf), Contains::None);

        let (left_elf, right_elf) = 
            parse_line("4-5,1-5".to_owned());
        assert_eq!(left_elf.contains(&right_elf), Contains::Right);

        let (left_elf, right_elf) = 
            parse_line("1-3,1-3".to_owned());
        assert_eq!(left_elf.contains(&right_elf), Contains::Left);
            // Edge case. When both ranges are the same, returns Left.
        

            
    }

    #[test]
    fn test_partial_contains() {
        let (left_elf, right_elf) = 
            parse_line("5-7,7-9".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), true);

        let (left_elf, right_elf) = 
            parse_line("2-8,3-7".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), true);
        
        let (left_elf, right_elf) = 
            parse_line("6-6,4-6".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), true);

        let (left_elf, right_elf) = 
            parse_line("2-6,4-8".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), true);

        let (left_elf, right_elf) = 
            parse_line("1-2,9-10".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), false);

        let (left_elf, right_elf) = 
            parse_line("100-101,0-2".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), false);

        let (left_elf, right_elf) = 
            parse_line("9-85,8-85".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), true);
        
        let (left_elf, right_elf) = 
            parse_line("0-0,1-1".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), false);
        
        let (left_elf, right_elf) = 
            parse_line("0-0,0-1".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), true);

        let (left_elf, right_elf) = 
            parse_line("2-4,6-8".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), false);

        let (left_elf, right_elf) = 
            parse_line("2-3,4-5".to_owned());
        assert_eq!(left_elf.partial_contains(&right_elf), false);
    }
}