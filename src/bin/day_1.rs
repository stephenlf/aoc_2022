/// Key topics covered:
/// * Structs, methods, derived traits
/// * io with the BufReader
/// * Test cases
/// * String.parse::<T>()

use std::{fs, io};
use std::io::BufRead;

#[derive(Clone)]
struct Elf {
    foods: Vec<u32>
}

impl Elf {
    /// New empty-handed elf
    fn new() -> Self {
        Self {foods: Vec::new()}
    }
    
    /// Finds the total number of calories of all food items carried by this elf.
    fn calories(self) -> u32 {
        self.foods.into_iter().sum()
    }
}

fn parse_input(file: fs::File) -> Vec<Elf> {
    let mut expedition: Vec<Elf> = Vec::with_capacity(243);
    expedition.push(Elf::new());
 
    
    // Open file
    let line_reader = io::BufReader::new(file).lines().into_iter()  ;

    // Read each line 
    // !! BETTER: for Ok(line) in line_reader...
    for line in line_reader {
        
        let line = line.unwrap();
        if line == String::from('\n') || line.is_empty() {
            expedition.push(Elf::new());
        }
        else {
            expedition.last_mut().unwrap().foods.push(line.parse::<u32>().unwrap());
        }
    // If line is integer -> add to Elf.foods
    // If line is \n -> push Elf to output vec and make new Elf
    }

    expedition

}

fn main() {
    let file = fs::File::open("inputs\\1.input.txt")
        .expect("Couldn't read input file :(");

    let expedition = parse_input(file).into_iter();
    let expedition_calories = expedition.map(|elf| elf.calories());

    
    let mut highest_three = vec![0;3];
    let mut min = highest_three.clone().into_iter().min().unwrap();
    let mut min_i = highest_three.clone().into_iter().position(|x| x == min).unwrap();
    for e in expedition_calories {
        if e > min {
            highest_three[min_i] = e;
            min = highest_three.clone().into_iter().min().unwrap();
            min_i = highest_three.clone().into_iter().position(|x| x == min).unwrap();
        
        }

    }
    println!("The highest three elf things are {:?} with a total calorie count of {}", &highest_three.clone(), highest_three.into_iter().sum::<u32>());
}

#[cfg(test)]
mod test_1 {

    use super::*;

    #[test]
    fn test_calories() -> () {
        let elf = Elf {foods: vec![1, 2, 3, 4]};
        assert_eq!(elf.calories(), 10_u32);
    }


}