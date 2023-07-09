use std::{path::PathBuf};

mod _day_5;
use _day_5::*;

fn main() {
    let mut lines 
        = read_to_lines(PathBuf::from("inputs/5.input.txt"));
    let mut port = shipping::Port::deserialize(&mut lines);

    println!("Dropping line: {:?}", lines.next());
    
    for line in lines {
        let (num_crates, origin, dest) = parse_instruction(line.unwrap());
        port.push_pop(num_crates, origin, dest);
    }

    println!("{:?}", port);

    for (i, mut dock) in port.docks.into_iter().enumerate() {
        print!("{}", dock.pop().unwrap());
    }
// --------------------------PT 2-----------------------
    println!("\n\nStarting part 2");

    let mut lines 
        = read_to_lines(PathBuf::from("inputs/5.input.txt"));
    let mut port = shipping::Port::deserialize(&mut lines);

    println!("(PT 2) Dropping line: {:?}", lines.next());
    
    for line in lines {
        let (num_crates, origin, dest) = parse_instruction(line.unwrap());
        port.push_pop_from_start(num_crates, origin, dest);
    }


    for (i, mut dock) in port.docks.into_iter().enumerate() {
        print!("{}", dock.pop().unwrap());
    }
}

#[cfg(test)]
mod test_5 {

}