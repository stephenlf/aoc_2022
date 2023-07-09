#[allow(unused)]

use std::fs;
use std::path::PathBuf;

fn all_unique(slice: &mut Vec<char>) -> bool {
    while let Some(c) = slice.pop() {
        if slice.contains(&c) { return false }
    }
    true
}

fn start_of_packet(signal: String) -> Option<usize> {
    let chars = signal.chars().collect::<Vec<char>>();

    let mut num_chars: usize = 4;

    for window in chars.windows(4) {
        let mut slice = window.to_owned();
        if all_unique(&mut slice) {
            return Some(num_chars)
        }
        num_chars += 1;
    }
    None
}

fn start_of_message(signal: String) -> Option<usize> {
    let chars = signal.chars().collect::<Vec<char>>();

    let mut num_chars: usize = 14;

    for window in chars.windows(14) {
        let mut slice = window.to_owned();
        if all_unique(&mut slice) {
            return Some(num_chars)
        }
        num_chars += 1;
    }
    None
}

fn main() {
    let input = fs::read_to_string(PathBuf::from("inputs/6.input.txt"))
        .unwrap();
    println!("Signal start at: {}", start_of_packet(input).unwrap());
    
    let input = fs::read_to_string(PathBuf::from("inputs/6.input.txt"))
        .unwrap();
    println!("Message start at: {}", start_of_message(input).unwrap());
}

#[cfg(test)]
mod day_6_tests {
    use super::*;

    #[test]
    fn test_all_unique() {
        let mut unique_slice = vec!['a','b','c','d'];
        let mut repeated_slice = vec!['a','b','c','a'];
        assert_eq!(all_unique(&mut unique_slice), true);
        assert_eq!(all_unique(&mut repeated_slice), false);
    }

    #[test]
    fn test_start_of_packet() {
        let s1 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let s2 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let s3 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let s4 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(start_of_packet(s1), Some(5));
        assert_eq!(start_of_packet(s2), Some(6));
        assert_eq!(start_of_packet(s3), Some(10));
        assert_eq!(start_of_packet(s4), Some(11));
    }
}

// Sliding window iterator.
// a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p
//[a,b,c,d].......................
//..[b,c,d,e].....................
//....[c,d,e,f]...................

// 1) Sliding window frame. DONE
// 2) Character check... are all entries of vector slice unique?