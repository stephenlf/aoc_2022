pub mod part_2;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum PosH {
    Up,
    UR,
    Ri,
    DR,
    Do,
    DL,
    Le,
    UL,
    Md,
}

#[derive(Clone, Copy, Debug)]
enum MovH {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for MovH {
    type Error = InvalidCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'U' => Ok(Self::Up),
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            _ => Err(InvalidCharError(value))
        }
    }
}

#[derive(Debug)]
pub struct InvalidCharError(char);
impl std::error::Error for InvalidCharError {}
impl std::fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot parse char {} as direction", self.0)
    }
}

pub struct StateMachine {
    abs_t: (isize, isize),      // Absolute position of tail
    rel_h: PosH,                // Relative position of head
    pub visited: HashSet<(isize, isize)>,   // Set of all previously visited locations
}
impl StateMachine {
    pub fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0,0));
        Self {    
            abs_t: (0,0),
            rel_h: PosH::Md,
            visited,
        }    
    }

    fn parse_line(s: String) -> (MovH, u32) {
        let mut t = s.split_whitespace();
        let dir = t.next()
        .unwrap()
        .chars()
        .next()
        .unwrap()
        .try_into()
        .unwrap();
        let rep = t.last().unwrap().parse().unwrap();
        println!("{:?}",&(dir, rep));
        (dir, rep)
    }

    fn step(&mut self, direction: MovH, num_steps: u32) {
        let (dx, dy) = match (self.rel_h, direction) {
            (PosH::Up, MovH::Up) => {
                (0, 1)
            }
            (PosH::Up, MovH::Left) => {
                self.rel_h = PosH::UL;
                (0,0)
            }
            (PosH::Up, MovH::Right) => {
                self.rel_h = PosH::UR;
                (0,0)
            }
            (PosH::UR, MovH::Up) => {
                self.rel_h = PosH::Up;
                (1,1)
            }
            (PosH::UR, MovH::Down) => {
                self.rel_h = PosH::Ri;
                (0,0)
            }
            (PosH::UR, MovH::Left) => {
                self.rel_h = PosH::Up;
                (0,0)
            }
            (PosH::UR, MovH::Right) => {
                self.rel_h = PosH::Ri;
                (1,1)
            }
            (PosH::Ri, MovH::Up) => {
                self.rel_h = PosH::UR;
                (0,0)
            }
            (PosH::Ri, MovH::Down) => {
                self.rel_h = PosH::DR;
                (0,0)
            }
            (PosH::Ri, MovH::Left) | (PosH::Up, MovH::Down) | (PosH::Do, MovH::Up) | (PosH::Le, MovH::Right) => {
                self.rel_h = PosH::Md;
                (0,0)
            }
            (PosH::Ri, MovH::Right) => {
                (1,0)
            }
            (PosH::DR, MovH::Up) => {
                self.rel_h = PosH::Ri;
                (0,0)
            }
            (PosH::DR, MovH::Down) => {
                self.rel_h = PosH::Do;
                (1,-1)
            }
            (PosH::DR, MovH::Left) => {
                self.rel_h = PosH::Do;
                (0,0)
            }
            (PosH::DR, MovH::Right) => {
                self.rel_h = PosH::Ri;
                (1,-1)
            }
            (PosH::Do, MovH::Down) => {
                (0, -1)
            }
            (PosH::Do, MovH::Left) => {
                self.rel_h = PosH::DL;
                (0,0)
            }
            (PosH::Do, MovH::Right) => {
                self.rel_h = PosH::DR;
                (0,0)
            }
            (PosH::DL, MovH::Up) => {
                self.rel_h = PosH::Le;
                (0,0)
            }
            (PosH::DL, MovH::Down) => {
                self.rel_h = PosH::Do;
                (-1,-1)
            }
            (PosH::DL, MovH::Left) => {
                self.rel_h = PosH::Le;
                (-1,-1)
            }
            (PosH::DL, MovH::Right) => {
                self.rel_h = PosH::Do;
                (0,0)
            }
            (PosH::Le, MovH::Up) => {
                self.rel_h = PosH::UL;
                (0,0)
            }
            (PosH::Le, MovH::Down) => {
                self.rel_h = PosH::DL;
                (0,0)
            }
            (PosH::Le, MovH::Left) => {
                (-1,0)
            }
            (PosH::UL, MovH::Up) => {
                self.rel_h = PosH::Up;
                (-1,1)
            }
            (PosH::UL, MovH::Down) => {
                self.rel_h = PosH::Le;
                (0,0)
            }
            (PosH::UL, MovH::Left) => {
                self.rel_h = PosH::Le;
                (-1,1)
            }
            (PosH::UL, MovH::Right) => {
                self.rel_h = PosH::Up;
                (0,0)
            }
            (PosH::Md, MovH::Up) => {
                self.rel_h = PosH::Up;
                (0,0)
            }
            (PosH::Md, MovH::Down) => {
                self.rel_h = PosH::Do;
                (0,0)
            }
            (PosH::Md, MovH::Left) => {
                self.rel_h = PosH::Le;
                (0,0)
            }
            (PosH::Md, MovH::Right) => {
                self.rel_h = PosH::Ri;
                (0,0)
            }
        };
        let (x, y) = self.abs_t;
        self.abs_t = (x + dx, y + dy);
        self.visited.insert(self.abs_t.clone());
    }

    pub fn execute_line(&mut self, s: String) {
        let (direction, num_steps) = Self::parse_line(s);
        for _ in 0..num_steps {
            self.step(direction, num_steps);
        }
    }
}

#[cfg(test)]
mod structs_9_test {
    use super::*;

    #[test]
    fn step() {
        let state = StateMachine::new();
    }
}