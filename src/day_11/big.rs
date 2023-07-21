use std::{rc::Rc, cell::RefCell, io::{Lines, BufReader}, fs::File, collections::VecDeque};
use num::{integer};

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(u64)
}

#[derive(Debug)]
pub struct PreMonkey {
    id: usize,
    items: VecDeque<u64>,
    operator: Operator,       // '*' or '+'
    operand: Operand, 
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
    pub examined: u64,
}

impl Monkey {
    pub fn list_items(&self) {
        println!("Monkey {}: [{}] {:?}", self.id, self.examined, self.items);
    }

    pub fn throw_items(&mut self) -> Vec<(u64, usize)> {
        let mut checked_items: Vec<(u64, usize)> = Vec::new();

        while let Some(item) = self.items.pop_front() {
            self.examined += 1;
            let concern = (self.operation)(item);
            match (self.test)(concern) {
                true => checked_items.push((concern, self.true_monkey)),
                false => checked_items.push((concern, self.false_monkey)),
            }
        }
        checked_items
    }
}

pub fn parse_lines(path: &'static str) -> Vec<Rc<RefCell<Monkey>>> {
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();
    
    let premonkeys = parse_premonkeys(path);

    let divisors_lcm = lcm_iter(
        &premonkeys.iter()
        .map(|premonkey|premonkey.divisor)
        .collect::<Vec<u64>>());

    println!("{divisors_lcm}");

    for premonkey in premonkeys {
        monkeys.push(Rc::new(RefCell::new(Monkey { 
            id: premonkey.id, 
            items: premonkey.items, 
            operation: build_operation(premonkey.operator, premonkey.operand, divisors_lcm), 
            test: build_test(premonkey.divisor), 
            true_monkey: premonkey.true_monkey, 
            false_monkey: premonkey.false_monkey,
            examined: 0,
        })))
    }

    monkeys
}

fn build_test(divisor: u64) -> Box<dyn Fn(u64) -> bool> {
    Box::new(
        move |x| x % divisor == 0
    )
}

fn build_operation(operator: Operator, operand: Operand, lcm: u64) -> Box<dyn Fn(u64) -> u64> {
    match operator {
        Operator::Times => match operand {
            Operand::Old => Box::new(move |x| x.pow(2) % lcm),
            Operand::Number(n) => Box::new(move |x| (x * n) % lcm),
        }
        Operator::Plus => match operand {
            Operand::Old => Box::new(move |x| (x * 2) % lcm),
            Operand::Number(n) => Box::new(move |x| (x + n) % lcm),
        }
    }
}

fn lcm_iter(nums: &Vec<u64>) -> u64 {
    let mut lcm = 1;
    for num in nums {
        lcm = integer::lcm(lcm, *num);
    }
    lcm
}

fn parse_premonkeys(path: &'static str) -> Vec<PreMonkey> {
    let mut premonkeys: Vec<PreMonkey> = Vec::new();
    let mut lines = super::lines(path);

    while let Some(pre_monkey) = parse_premonkey(&mut lines) {
        premonkeys.push(pre_monkey);
    }

    premonkeys
}

fn parse_premonkey(lines: &mut Lines<BufReader<File>>) -> Option<PreMonkey> {
    let mut s = lines.next()?.unwrap();                 // Monkey 0: 
    let id = parse_id(s);

    s = lines.next()?.unwrap();
    let items = parse_items(s);

    s = lines.next()?.unwrap();
    let (operator, operand) = parse_operation(s);

    s = lines.next()?.unwrap();
    let divisor = parse_divisor(s);

    s = lines.next()?.unwrap();
    let true_monkey = parse_true_monkey(s);

    s = lines.next()?.unwrap();
    let false_monkey = parse_false_monkey(s);

    let _ = lines.next();
    
    Some(PreMonkey { id, 
        items, 
        operator, 
        operand, 
        divisor, 
        true_monkey, 
        false_monkey })
}

fn parse_false_monkey(s: String) -> usize {
    assert_eq!(&s.trim()[..26], "If false: throw to monkey ");
    s.trim()[26..].parse().unwrap()
}

fn parse_true_monkey(s: String) -> usize {
    assert_eq!(&s.trim()[..25], "If true: throw to monkey ");
    s.trim()[25..].parse().unwrap()
}

fn parse_divisor(s: String) -> u64 {
    assert_eq!(&s.trim()[..19], "Test: divisible by ");
    s.trim()[19..].parse().unwrap()
}

fn parse_operation(s: String) -> (Operator, Operand) {
    assert_eq!(&s.trim()[..21], "Operation: new = old ");
    
    let operator =  match s.trim().chars().nth(21).unwrap() {
        '*' => Operator::Times,
        '+' => Operator::Plus,
        _ => panic!(),
    };

    let operand: Operand = match &s.trim()[23..] {
        "old" => Operand::Old,
        n => Operand::Number(n.parse().unwrap())
    };

    (operator, operand)
}

fn parse_id(s: String) -> usize {
    // "Monkey 0:"
    assert_eq!(&s.trim()[..7], "Monkey ");
    s.trim()                            // [&str]   Remove whitespace
        .chars().nth(7).unwrap()        // [char]   Get 7th character
        .to_digit(10).unwrap()    // [u64]    Convert to decimal digit
        .try_into().unwrap()                // [usize]  Convert to usize
}

fn parse_items(s: String) -> VecDeque<u64> {
    assert_eq!(&s.trim()[..16], "Starting items: ");
    let items = s.trim()[16..]
        .split_terminator(',')      
        .map(|slice| slice
            .trim()
            .parse::<u64>()
            .unwrap())
        .collect::<VecDeque<u64>>();

    items
}

#[cfg(test)]
mod big_11_tests {
    use super::*;
    #[test]
    fn throw_items_test() {

    }
    
    #[test]
    fn parse_lines_test() {

    }
    
    #[test]
    fn build_test_test() {
        let test = build_test(7);
        assert_eq!(test(7), true);
        assert_eq!(test(14), true);
        assert_eq!(test(1), false);
        assert_eq!(test(9), false);

        let test = build_test(8);
        assert_eq!(test(4), false);
        assert_eq!(test(8), true);
    }
    
    #[test]
    fn build_operation_test() {
        let operation = build_operation(Operator::Plus, Operand::Number(4), 100);
        assert_eq!(operation(8), 12);
        assert_ne!(operation(5), 4);

        let operation = build_operation(Operator::Plus, Operand::Old, 100);
        assert_eq!(operation(8), 16);
        assert_ne!(operation(5), 5);

        let operation = build_operation(Operator::Times, Operand::Number(6), 100);
        assert_eq!(operation(4), 24);
        assert_ne!(operation(5), 29);

        let operation = build_operation(Operator::Times, Operand::Old, 100);
        assert_eq!(operation(5), 25);
        assert_ne!(operation(4), 25);
    }
    
    #[test]
    fn lcm_iter_test() {
        let lcm = lcm_iter(&vec![1, 2, 3]);
        assert_eq!(lcm, 6);

        let lcm = lcm_iter(&vec![2,3,4]);
        assert_eq!(lcm, 12);

        let lcm = lcm_iter(&vec![2,3,5]);
        assert_eq!(lcm, 30);
    }
    
    #[test]
    fn parse_premonkeys_test() {

    }
    
    #[test]
    fn parse_premonkey_test() {

    }
    
    #[test]
    fn parse_false_monkey_test() {

    }
    
    #[test]
    fn parse_true_monkey_test() {

    }
    
    #[test]
    fn parse_divisor_test() {

    }
    
    #[test]
    fn parse_operation_test() {

    }
    
    #[test]
    fn parse_id_test() {

    }
    
    #[test]
    fn parse_items_test() {

    }
    
}