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
    Number(u32)
}

#[derive(Debug)]
pub struct PreMonkey {
    id: usize,
    items: VecDeque<u32>,
    operator: Operator,       // '*' or '+'
    operand: Operand, 
    divisor: u32,
    true_monkey: usize,
    false_monkey: usize,
}

pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    test: Box<dyn Fn(u32) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
    pub examined: u32,
}

impl Monkey {
    pub fn list_items(&self) {
        println!("Monkey {}: {:?}", self.id, self.items);
    }

    pub fn throw_items(&mut self) -> Vec<(u32, usize)> {
        let mut checked_items: Vec<(u32, usize)> = Vec::new();

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
        .collect::<Vec<u32>>());

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

fn build_test(divisor: u32) -> Box<dyn Fn(u32) -> bool> {
    Box::new(
        move |x| x % divisor == 0
    )
}

fn build_operation(operator: Operator, operand: Operand, lcm: u32) -> Box<dyn Fn(u32) -> u32> {
    match operator {
        Operator::Times => match operand {
            Operand::Old => Box::new(move |x| x.pow(2) % lcm),
            Operand::Number(n) => Box::new(move |x| (x * 2) % lcm),
        }
        Operator::Plus => match operand {
            Operand::Old => Box::new(move |x| (x * 2) % lcm),
            Operand::Number(n) => Box::new(move |x| (x + n) % lcm),
        }
    }
}

fn lcm_iter(nums: &Vec<u32>) -> u32 {
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

fn parse_divisor(s: String) -> u32 {
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
        .to_digit(10).unwrap()    // [u32]    Convert to decimal digit
        .try_into().unwrap()                // [usize]  Convert to usize
}

fn parse_items(s: String) -> VecDeque<u32> {
    assert_eq!(&s.trim()[..16], "Starting items: ");
    let items = s.trim()[16..]
        .split_terminator(',')      
        .map(|slice| slice
            .trim()
            .parse::<u32>()
            .unwrap())
        .collect::<VecDeque<u32>>();

    items
}