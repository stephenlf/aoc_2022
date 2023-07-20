/// Key concepts:
/// * Closure types
/// * Double-ended queue (VecDeque)
/// * Custom ordering with "use std::cmp::Ordering;"
pub mod big;
use std::collections::VecDeque;
use std::error::Error;
use std::{rc::Rc, cell::RefCell};
use super::lines;
use std::fmt::Debug;

pub fn parse_lines(path: &'static str) -> Result<Vec<Rc<RefCell<Monkey>>>, Box<dyn Error>> {

    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    let mut lines = lines(path);
    while let Some(s) = lines.next() {
        
        let id = {
            let mut s = s.unwrap();                          // "Monkey 0:"
            let mut t = s.split_whitespace();   // ["Monkey", "0:"]
            assert_eq!(t.next().unwrap(), "Monkey");                 // "Monkey"
            let next = t.next().unwrap();                      // "0:"
            next[..next.len()-1].parse::<usize>().unwrap()           // "0".parse::<usize>() == 0_usize
        };

        let items: VecDeque<u128> = {
            let mut v: VecDeque<u128> = VecDeque::new();                     
            let s = lines.next().unwrap().unwrap();                 // "  Starting items: 66, 59, 64, 51"
            assert_eq!(&s.trim()[0..15], "Starting items:");                // "Starting items:"
            for n in s.trim_start()[15..].split_terminator(", ") {    // [" 66", " 59", " 64", "51"]
                v.push_back(n.trim().parse::<u128>().unwrap());              // v.push(66)
            }
            v
        };

        let operation: Box<dyn Fn(u128) -> u128> = {
            let s = lines.next().unwrap().unwrap();                // "  Operation: new = old * 19"
            assert_eq!(&s.trim()[..20], "Operation: new = old");

            let operator = &s.trim().chars().nth(21).unwrap();      // '*' or '+'
            assert!(operator == &'*' || operator == &'+');

            let term = &s.trim()[23..];                              // "3" or "old"
            match term {                                                   // This would be simpler with macros
                "old" => {
                    match operator {
                        
                        &'*' => Box::new(|x| x.pow(2)),
                        &'+' => Box::new(|x| x * 2_u128),
                        o => Err(format!("Invalid operator {o}"))?
                    }
                }
                n => {
                    let n = n.parse::<u128>().unwrap();
                    match operator {
                        &'*' => Box::new(move |x| x * n),               // Errors I had: "Borrowed value doesn't live long enough"
                        &'+' => Box::new(move |x| x + n),               // "`match` arms have incompatible types (no two closures have the same type)"
                        o => Err(format!("Invalid operator {o}"))?    // Couldn't I just store this operation in the impl block? Yes.
                    }                                                        // "Add ?Sized to monkey definition"
                }
            }
        };

        let test: Box<dyn Fn(&u128) -> bool> = {
            let s = lines.next().unwrap().unwrap();             // "  Test: divisible by 5"
            assert_eq!(&s.trim()[..18], "Test: divisible by");

            let divisor = s.trim()[19..].parse::<u128>().unwrap();   // "5"
            Box::new(move |x: &u128| {
                let test = x % divisor.clone() == 0_u32.into();
                test
            })
        };

        let true_monkey: usize = {
            let s = lines.next().unwrap().unwrap();             // "    If true: throw to monkey 3"
            assert_eq!(&s.trim()[..24], "If true: throw to monkey");
            s.trim()[25..].parse::<usize>().unwrap()                    // "3"
        };
        
        let false_monkey: usize = {
            let s = lines.next().unwrap().unwrap();             // "    If false: throw to monkey 3"
            assert_eq!(&s.trim()[..25], "If false: throw to monkey");
            s.trim()[26..].parse::<usize>().unwrap()                    // "3"
        };

        let _ = lines.next();                                           // "\n"

        let monkey = Monkey {
            id,
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
            checked: 0_u8.into(),
        };

        monkeys.push(Rc::new(RefCell::new(monkey)));
    }
    
    Ok(monkeys)
}

pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(&u128) -> bool>,
    true_monkey: usize,     // We could also use Rc<RefCell<Monkey<F, G>>>
    false_monkey: usize,    // We could also use Rc<RefCell<Monkey<F, G>>>
    pub checked: u128,  
}

impl Monkey {
    pub fn throw_items(&mut self) -> Vec<(u128, usize)> {
        let mut checked_items: Vec<(u128, usize)> = Vec::new();

        while let Some(item) = self.items.pop_front() {
            self.checked += 1_u128;
            //let concern = (self.operation)(item) / 3;
            let concern = (self.operation)(item);
            // If concern.1 {concern = concern + u128::MAX * }
            match (self.test)(&concern) {
                true => checked_items.push((concern, self.true_monkey)),
                false => checked_items.push((concern, self.false_monkey)),
            }
        }
        checked_items
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {{ id: {}, items: {:?}, true_monkey: {}, false_monkey: {} }}",
        self.id, self.items, self.true_monkey, self.false_monkey)
    }
}

#[cfg(test)]
mod day_11 {
    use super::*;
    #[test]
    fn parse_line() {
        let monkeys = parse_lines("inputs/11.inputs.txt").unwrap();
        for monkey in &monkeys {
            println!("{:?}",monkey);
        }
        assert_eq!((&monkeys[0].borrow().operation)(2_u8.into()), 6_u8.into());
        assert_eq!((&monkeys[0].borrow().operation)(4_u8.into()), 12_u8.into());
        assert_eq!((&monkeys[2].borrow().operation)(2_u8.into()), 4_u8.into());
        assert_eq!((&monkeys[2].borrow().operation)(6_u8.into()), 8_u8.into());

        assert_eq!((&monkeys[0].borrow().test)(&2_u8.into()), true);
        assert_eq!((&monkeys[0].borrow().test)(&1_u8.into()), false);
        assert_eq!((&monkeys[1].borrow().test)(&1_u8.into()), false);
        assert_eq!((&monkeys[1].borrow().test)(&8_u8.into()), false);
        assert_eq!((&monkeys[1].borrow().test)(&14_u8.into()), true);
    }
}
/*
error[E0277]: `dyn Fn(u32) -> bool` doesn't implement `Debug`
  --> src\bin\day_11.rs:96:26
   |
96 |         println!("{:?}", monkey)
   |                          ^^^^^^ `dyn Fn(u32) -> bool` cannot be formatted using `{:?}` because it doesn't implement `Debug`

   Solution: impl Debug for Monkey.

Issues with multiple mutability.

thread 'main' panicked at 'attempt to multiply with overflow', src\day_11\mod.rs:46:46
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    solution: change u32 to u128
    Used x.overflowing_mul(n) and x.overflowing_add(n)
        Refactored code to track number of overflows.

    [unnessesary. Should have also changed usize to u128]

cargo add num -F num-bigint
   */

/*
Why use crates?

Rust is a systems language, meaning lots of stuff that is normally provided as language primitives or in std aren't included in Rust
(See async/await documentation, rust book web server project). Examples: HTTP functionality. Anything to do with async/await (except the keywords
and the futures type). Big integers/some math operations. 

I tried the num::BigUint crate, but it slowed down the program too much.
*/