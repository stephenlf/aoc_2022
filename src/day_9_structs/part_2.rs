use core::num;
use std::collections::HashSet;
pub use std::{rc::Rc, cell::RefCell};

#[derive(Clone, Default, Debug)]
pub struct Knot {
    position: (i32, i32),
}

impl Knot {
    pub fn new() -> Self {
        Self {position: (0,0)}
    }

    fn calc_dxy(child_pos: (i32, i32), parent_pos: (i32, i32)) -> (i32, i32) {
        let (self_x, self_y) = child_pos;
        let (parent_x, parent_y) = parent_pos;
        let dx = parent_x - self_x;
        let dy = parent_y - self_y;
        
        if dx.abs() < 2 && dy.abs() < 2 {
            return (0,0)
        } 

        let tail_dx = if dx.is_positive() {1} 
            else if dx.is_negative() {-1} 
            else {0};
        let tail_dy = if dy.is_positive() {1} 
            else if dy.is_negative() {-1} 
            else {0};
        (tail_dx, tail_dy)
    }

    /// Takes in a single step in the form of (x, y). Validates that 
    /// step is of magnitude 1 [e.g. (1,0), (0,1), (-1,0), (0,-1)]
    /// and shifts knot's position by that much. Returns updated 
    /// knot position.
    pub fn update_rel(&mut self, rel_pos: (i32, i32)) -> (i32, i32) {
        let (dx, dy) = rel_pos;
        assert!(dx >= -1 && dx <= 1);
        assert!(dy >= -1 && dy <= 1);
        self.position.0 += dx;
        self.position.1 += dy;
        (self.position.0, self.position.1)
    }

    /// Takes in the absolute position of the parent knot and updates the child 
    /// position accordingly. Returns the new position of the child knot.
    pub fn update_pos(&mut self, parent_pos: (i32, i32)) -> (i32, i32) {
        let (dx, dy) = Knot::calc_dxy(self.position, parent_pos);
        self.position.0 += dx;
        self.position.1 += dy;
        (self.position.0, self.position.1)
    }
}

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Knot>,
    visited_locations: HashSet<(i32, i32)>,
}

impl Rope {
    pub fn new(num_knots: usize) -> Self {
        let mut hs: HashSet<(i32, i32)> = HashSet::new();
        hs.insert((0,0));
        Self {
            knots: vec![Knot::new(); num_knots],
            visited_locations: hs,
        }
    }

    fn parse_line(s: String) -> ((i32, i32), usize) {
        let mut s = s.split_whitespace();
        let delta: (i32, i32) = match s.next().unwrap() {
            "R" => (1,0),
            "L" => (-1,0),
            "U" => (0,1),
            "D" => (0,-1),
            c => panic!("Problem parsing {c} as direction (expect U, L, R, or D)"),
        };
        let num_steps = s.last().unwrap().parse::<usize>().unwrap();
        (delta, num_steps)
    }

    fn iter_step(&mut self, mut delta: (i32, i32)) -> (i32, i32) {
        let mut position = (0,0);
        for (i, knot) in &mut self.knots.iter_mut().enumerate() {
            if i == 0 {
                position = knot.update_rel(delta);
            } else {
                position = knot.update_pos(position);
            }
        }
        position
    }

    pub fn step(&mut self, s: String) {
        let (delta, num_steps) = Self::parse_line(s);
        for _ in 0..num_steps {
            let tail_position = self.iter_step(delta);
            self.visited_locations.insert(tail_position);
        }
    }

    pub fn size(&self) -> usize {
        self.visited_locations.len()
    }
}

#[cfg(test)]
mod test_9_2 {
    use super::*;

    #[test]
    fn calc_dxy() {
        assert_eq!(Knot::calc_dxy((0,0), (0,0)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (1,0)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (1,1)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (0,1)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (-1,0)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (-1,-1)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (0,-1)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (1,-1)), (0,0));
        assert_eq!(Knot::calc_dxy((0,0), (-1,1)), (0,0));

        assert_eq!(Knot::calc_dxy((0,0), (2,0)), (1,0));
        assert_eq!(Knot::calc_dxy((0,0), (2,1)), (1,1));
        assert_eq!(Knot::calc_dxy((0,0), (2,2)), (1,1));
        assert_eq!(Knot::calc_dxy((0,0), (1,2)), (1,1));
        assert_eq!(Knot::calc_dxy((0,0), (0,2)), (0,1));
        assert_eq!(Knot::calc_dxy((0,0), (-1,2)), (-1,1));
        assert_eq!(Knot::calc_dxy((0,0), (-2,2)), (-1,1));
        assert_eq!(Knot::calc_dxy((0,0), (-2,1)), (-1,1));
        assert_eq!(Knot::calc_dxy((0,0), (-2,0)), (-1,0));
        assert_eq!(Knot::calc_dxy((0,0), (-2,-1)), (-1,-1));
        assert_eq!(Knot::calc_dxy((0,0), (-2,-2)), (-1,-1));
        assert_eq!(Knot::calc_dxy((0,0), (-1,-2)), (-1,-1));
        assert_eq!(Knot::calc_dxy((0,0), (0,-2)), (0,-1));
        assert_eq!(Knot::calc_dxy((0,0), (1,-2)), (1,-1));
        assert_eq!(Knot::calc_dxy((0,0), (2,-2)), (1,-1));
        assert_eq!(Knot::calc_dxy((0,0), (2,-1)), (1,-1));
    }

    #[test]
    fn test_update_pos() {
        let mut knot = Knot {
            position: (0,0)
        };
        knot.update_pos((1,1));
        assert_eq!(knot.position, (0,0));
        knot.update_pos((2,2));
        assert_eq!(knot.position, (1,1));
    }

    #[test]
    fn test_rel_update() {
        let mut knot = Knot::new();
        assert_eq!(knot.position, (0,0));
        knot.update_rel((1,0));
        assert_eq!(knot.position, (1,0));
        knot.update_rel((1,0));
        assert_eq!(knot.position, (2,0));
        knot.update_rel((-1,0));
        assert_eq!(knot.position, (1,0));
        knot.update_rel((0,-1));
        assert_eq!(knot.position, (1,-1));

    }

    #[test]
    fn test_parse_line() {
        assert_eq!(Rope::parse_line("U 13".to_string()), ((0,1), 13));
        assert_eq!(Rope::parse_line("D 13".to_string()), ((0,-1), 13));
        assert_eq!(Rope::parse_line("L 13".to_string()), ((-1,0), 13));
        assert_eq!(Rope::parse_line("R 13".to_string()), ((1,0), 13));
    }

    #[test]
    fn test_iter_step() {
        let mut rope = Rope::new(2);
        assert_eq!(rope.iter_step((1,0)), (0,0));
        assert_eq!(rope.iter_step((1,0)), (1,0));
        assert_eq!(rope.iter_step((0,1)), (1,0));
        assert_eq!(rope.iter_step((0,1)), (2,1));
    }

    #[test]
    fn test_step() {
        let mut rope = Rope::new(2);
        rope.step("U 4".to_string());
        assert_eq!(rope.visited_locations.len(), 4);
    }
}