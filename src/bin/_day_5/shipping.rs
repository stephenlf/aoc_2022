#![allow(unused)]

use super::read_to_lines;
use core::num;
use std::collections::HashMap;
pub use std::{path::PathBuf,io::{Lines, BufReader}, fs::File};

#[derive(Debug, Clone, Default)]
pub struct Port {
    pub docks: Vec<Vec<char>>,
}


// port[dock][crate]
// port[dock].pop()//.push()

impl Port {
    pub fn deserialize(lines: &mut Lines<BufReader<File>>) -> Self {
        let mut port = Port { docks: vec![Vec::new();9] };

        let mut temp_ports: Vec<String> = Vec::new();

        for line in lines {
            let line = line.unwrap();
            if line.split_ascii_whitespace().next().unwrap() == "1" {
                temp_ports.push(line);
                break
            }
            temp_ports.push(line);
        }

        let mut temp_docks: HashMap<usize, u32> = HashMap::new();        // {character_index, dock_number}
        
        for (i, c) in temp_ports.last().unwrap().chars().enumerate() {
            if c.is_digit(10) {
                temp_docks.insert(i, c.to_digit(10).unwrap());
            }
        }

        temp_ports.pop();

        for string in temp_ports {
            for (i, c) in string.chars().enumerate() {
                let dock_number = (temp_docks.get(&i));

                if temp_docks.contains_key(&i) && c.is_alphabetic() {
                    if let Some(n) = dock_number {
                        let mut n: usize = (*n).try_into().unwrap();
                        n -= 1;
                        port.docks[n].push(c);
                    }
                }
            }
        }

        port.docks = port.docks
            .clone()
            .into_iter()
            .map(|mut d| {
                d.reverse();
                d})
            .collect();

        port
    }

    pub fn push_pop(&mut self, num_crates: u32, origin: usize, dest: usize) {
        // for i in num_crates {dest.push(orign.pop())}
        for i in 0..num_crates {
            let container_result = self.docks[origin - 1].pop();
            if let Some(c) = container_result {
                self.docks[dest - 1].push(c);
            } else {
                println!("push_pop failed at x, y, z: {}, {}, {}", num_crates, origin, dest);
            }
            
            ()
        }
    }

    pub fn push_pop_from_start(&mut self, num_crates: u32, origin: usize, dest: usize) {
        // for i in num_crates {dest.push(orign.pop())}
        let num_crates: usize = num_crates.try_into().unwrap();

        let length = self.docks[origin-1].len();

        let mut stack = self.docks[origin-1]
            .drain((length - num_crates)..)
            .collect::<Vec<char>>();

        self.docks[dest - 1].append(&mut stack);
    }
}

#[cfg(test)]
mod shipping_tests {
    use super::*;
    #[test]
    fn test_deserialize() {
        let mut lines 
            = read_to_lines(PathBuf::from("inputs/5.input.txt"));
        let port = Port::deserialize(&mut lines);
    }

    fn init_port() -> Port {
        Port {docks: 
            vec![
                vec!['A','B','C'],
                vec!['D','E','F'],
                vec!['G','H','I'],
            ]}
    }

    #[test]
    fn test_push_pop() {
        let mut port = init_port();
        port.push_pop(3, 1, 3);
        assert_eq!(port.docks[2], vec!['G','H','I','C','B','A']);
    }

    #[test]
    fn test_push_pop_from_start() {
        let mut port = init_port();
        port.push_pop_from_start(3, 1, 3);
        assert_eq!(port.docks[0].len(), 0);
        assert_eq!(port.docks[2], vec!['G','H','I','A','B','C']);
    }

}