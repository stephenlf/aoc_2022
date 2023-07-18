/// Key concepts:
/// * Closure types
/// * Double-ended queue (VecDeque)
use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;
use advent_of_code::lines;

fn parse_lines<F, G>(path: &'static str) -> Result<Vec<Monkey<F, G>>, Box<dyn Error>>
where F: Fn(u32) -> u32, G: Fn(u32) -> bool {
    println!("Hey");

    let mut monkeys: Vec<Monkey<F, G>> = Vec::new();

    let mut lines = lines(path);
    while let Some(s) = lines.next() {
        
        let id = {
            let mut s = s.unwrap();                          // "Monkey 0:"
            let mut t = s.split_whitespace();   // ["Monkey", "0:"]
            assert_eq!(t.next().unwrap(), "Monkey");                 // "Monkey"
            let next = t.next().unwrap();                      // "0:"
            next[..next.len()-1].parse::<usize>().unwrap()           // "0".parse::<usize>() == 0_usize
        };
        println!("{id}");

        let items: VecDeque<u32> = {
            let mut v: VecDeque<u32> = VecDeque::new();                     
            let s = lines.next().unwrap().unwrap();                 // "  Starting items: 66, 59, 64, 51"
            assert_eq!(&s.trim()[0..15], "Starting items:");                // "Starting items:"
            for n in s.trim_start()[15..].split_terminator(", ") {    // [" 66", " 59", " 64", "51"]
                v.push_back(n.trim().parse::<u32>().unwrap());              // v.push(66)
            }
            v
        };

        let operation: Box<dyn Fn(u32) -> u32> = {
            let s = lines.next().unwrap().unwrap();                // "  Operation: new = old * 19"
            assert_eq!(&s.trim()[..20], "Operation: new = old");

            let operator = &s.trim().chars().nth(21).unwrap();      // '*' or '+'
            assert!(operator == &'*' || operator == &'+');

            let term = &s.trim()[23..];                              // "3" or "old"
            match term {                                                   // This would be simpler with macros
                "old" => {
                    match operator {
                        &'*' => Box::new(|x| x * x),
                        &'+' => Box::new(|x| x + x),
                        o => Err(format!("Invalid operator {o}"))?
                    }
                }
                n => {
                    let n = n.parse::<u32>().unwrap();
                    match operator {
                        &'*' => Box::new(move |x| x * n),               // Errors I had: "Borrowed value doesn't live long enough"
                        &'+' => Box::new(move |x| x + n),               // "`match` arms have incompatible types (no two closures have the same type)"
                        o => Err(format!("Invalid operator {o}"))?    // Couldn't I just store this operation in the impl block? Yes.
                    }                                                        // "Add ?Sized to monkey definition"
                }
            }
        };

        let test: Box<dyn Fn(u32) -> bool> = {
            let s = lines.next().unwrap().unwrap();             // "  Test: divisible by 5"
            assert_eq!(&s.trim()[..18], "Test: divisible by");

            let divisor = s.trim()[19..].parse::<u32>().unwrap();   // "5"

            Box::new(move |x| x % divisor == 0)
        };

        let true_monkey: usize = {
            let s = lines.next().unwrap().unwrap();             // "    If true: throw to monkey 3"
            assert_eq!(&s.trim()[..25], "If true: throw to monkey");

            s.trim()[26..].parse::<usize>().unwrap()
        };

        let false_monkey: usize = {
            let s = lines.next().unwrap().unwrap();             // "    If false: throw to monkey 3"
            assert_eq!(&s.trim()[..26], "If true: throw to monkey");

            s.trim()[27..].parse::<usize>().unwrap()
        };

        let monkey = Monkey {
            id,
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
        };

        
    }
    
    Ok(monkeys)
}

#[derive(Debug)]
struct Monkey<F, G>
where F: Fn(u32) -> u32 + ?Sized, G: Fn(u32) -> bool + ?Sized {
    id: usize,
    items: VecDeque<u32>,
    operation: Box<F>,
    test: Box<G>,
    true_monkey: usize,     // We could also use Rc<RefCell<Monkey<F, G>>>
    false_monkey: usize,    // We could also use Rc<RefCell<Monkey<F, G>>>
}

impl<F, G> Monkey<F, G>
where F: Fn(u32) -> u32, G: Fn(u32) -> bool {
    fn check_item(&mut self) -> (u32, usize) {
        let item: u32 = 0;
        let monkey_id: usize = 0;

        // Pop from front of queue.
        // Calc change in concern.
        // Calc which monkey to throw to.
        // Return. Value will be taken by carrier struct and pushed to other monkey.

        (item, monkey_id)
    }
}

fn main() {
    
}

#[cfg(test)]
mod day_11 {
    use super::*;
    #[test]
    fn parse_line() {
        parse_lines::<&dyn Fn(u32) -> u32, &dyn Fn(u32) -> bool>("inputs/11.inputs.txt");
    }
}

/*
error[E0277]: `dyn Fn(u32) -> bool` doesn't implement `Debug`
  --> src\bin\day_11.rs:96:26
   |
96 |         println!("{:?}", monkey)
   |                          ^^^^^^ `dyn Fn(u32) -> bool` cannot be formatted using `{:?}` because it doesn't implement `Debug`

*/