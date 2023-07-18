/// Key concept:
/// String slices with &s[n..m]

use advent_of_code::day_10::*;
use advent_of_code::lines;

struct Clock{
    x: i32,
    cycle: i32,
    total: i32,
    buffer: String,
}

impl Clock {
    pub fn new() -> Self {
        Self { x: 1, cycle: 0, total: 0, buffer: String::new() }
    }

    pub fn noop(&mut self) {
        self.inc_cycle();
    }

    pub fn addx(&mut self, n: i32) {
        self.inc_cycle();
        self.inc_cycle();
        self.x += n;
    }

    fn inc_cycle(&mut self) {
        
        if ((self.cycle % 40) - self.x).abs() < 2 {
            self.buffer.push('#');
        } else {
            self.buffer.push('.');
        }
        
        self.cycle += 1;
        println!("x: {}, cycle: {}", self.x, self.cycle);

        if (self.cycle - 20) % 40 == 0 && self.cycle < 221 {
            self.total += self.x * self.cycle;
            println!("x: {}, cycle: {}, total: {}", self.x, self.cycle, self.total);
        }
    }
}

enum Command {
    Noop,
    Addx(i32),
    Error
}

fn parse_line(s: String) -> Command {
    let mut c = s.split_whitespace();
    match c.next().unwrap() {
        "noop" => Command::Noop,
        "addx" => Command::Addx(c.next().unwrap().parse::<i32>().unwrap()),
        _ => Command::Error,
    }
}

fn main() {
    let lines = lines("inputs/10.inputs.txt");

    let mut clock = Clock::new();
    
    for line in lines {
        match parse_line(line.unwrap()) {
            Command::Noop => clock.noop(),
            Command::Addx(n) => clock.addx(n),
            Command::Error => continue,
        }
    }
    println!("{}",&clock.buffer[0..39]);
    println!("{}",&clock.buffer[40..79]);
    println!("{}",&clock.buffer[80..119]);
    println!("{}",&clock.buffer[120..159]);
    println!("{}",&clock.buffer[160..199]);
    println!("{}",&clock.buffer[200..]);
}