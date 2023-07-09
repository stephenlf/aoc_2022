use std::fs;
use std::io::{BufRead,BufReader};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum OpponentMove {
    Rock,
    Paper,
    Scissors,
}

impl OpponentMove {
    fn parse_from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'A' => Self::Rock, // Rock
            'B' => Self::Paper, // Paper
            'C' => Self::Scissors, // Scissors
            _ => Self::Rock
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum MyMove {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

impl MyMove {
    fn parse_from_char(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'X' => Self::Rock(1),
            'Y' => Self::Paper(2),
            'Z' => Self::Scissors(3),
            _ => Self::Rock(1)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Score {
    Win,
    Tie,
    Lose
}

impl Score {
    fn value(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Tie => 3,
            Self::Lose => 0,
        }
    }
}

fn score(me: &MyMove, opponent: &OpponentMove) -> u32 {
    use MyMove::*;
    use Score::*;
    match (me, opponent) {
        (Rock(x), OpponentMove::Scissors) | (Paper(x), OpponentMove::Rock) | (Scissors(x), OpponentMove::Paper) => Win.value() + x,
        (Rock(x), OpponentMove::Paper) | (Paper(x), OpponentMove::Scissors) | (Scissors(x), OpponentMove::Rock) => Lose.value() + x,
        (Rock(x), _) | (Paper(x), _) | (Scissors(x), _) => Tie.value() + x,
    }
}

fn parse_2 (o: char, m: char) -> (OpponentMove, MyMove) {
    match (o, m) {
        ('A', 'X') => (OpponentMove::Rock, MyMove::Scissors(3)),
        ('A', 'Y') => (OpponentMove::Rock, MyMove::Rock(1)),
        ('A', 'Z') => (OpponentMove::Rock, MyMove::Paper(2)),
        ('B', 'X') => (OpponentMove::Paper, MyMove::Rock(1)),
        ('B', 'Y') => (OpponentMove::Paper, MyMove::Paper(2)),
        ('B', 'Z') => (OpponentMove::Paper, MyMove::Scissors(3)),
        ('C', 'X') => (OpponentMove::Scissors, MyMove::Paper(2)),
        ('C', 'Y') => (OpponentMove::Scissors, MyMove::Scissors(3)),
        ('C', 'Z') => (OpponentMove::Scissors, MyMove::Rock(1)),
        _ => ( OpponentMove::Paper, MyMove::Rock(0)),
    }
}

fn serialize_input() -> Vec<(OpponentMove, MyMove)> {
    let mut moves: Vec<(OpponentMove, MyMove)> = Vec::new();
    let input = fs::File::open("inputs\\2.input.txt").unwrap();
    let lines = BufReader::new(input).lines();
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();
        let opponent_move = OpponentMove::parse_from_char(chars.next().unwrap());
        chars.next();
        let my_move = MyMove::parse_from_char(chars.next().unwrap());
        moves.push((opponent_move, my_move));
    }
    moves
}

fn serialize_input_2() -> Vec<(OpponentMove, MyMove)> {
    let mut moves: Vec<(OpponentMove, MyMove)> = Vec::new();
    let input = fs::File::open("inputs\\2.input.txt").unwrap();
    let lines = BufReader::new(input).lines();
    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();
        let opponent_move = chars.next().unwrap();
        chars.next();
        let my_move = chars.next().unwrap();
        moves.push(parse_2(opponent_move, my_move));
    }
    moves
}

fn main() {
    let moves = serialize_input();
    let scores: Vec<u32> = moves.into_iter().map(|(o, m)| score(&m, &o)).collect();
    println!("Score (1): {:?}", scores.into_iter().sum::<u32>());
    
    let moves = serialize_input_2();
    let scores: Vec<u32> = moves.into_iter().map(|(o, m)| score(&m, &o)).collect();
    println!("Score (2): {:?}", scores.into_iter().sum::<u32>());

}

#[cfg(test)]
mod tests_2 {
    use super::*;

    const test_array: [(char, char);9] = [
        ('A', 'X'),
        ('A', 'Y'),
        ('A', 'Z'),
        ('B', 'X'),
        ('B', 'Y'),
        ('B', 'Z'),
        ('C', 'X'),
        ('C', 'Y'),
        ('C', 'Z'),
    ];

    fn parse_test_array() -> [(OpponentMove, MyMove); 9] {
        let mut parsed_array = [(OpponentMove::Rock, MyMove::Rock(1));9];
        for i in 0..9 {
            let (o, m) = test_array[i];
            parsed_array[i] = (OpponentMove::parse_from_char(o), MyMove::parse_from_char(m));
        }
        parsed_array
    }

    #[test]
    fn test_parse() {
        let parsed_array = parse_test_array();
        assert_eq!(parsed_array[0], (OpponentMove::Rock, MyMove::Rock(1)));
        assert_eq!(parsed_array.last().unwrap(), &(OpponentMove::Scissors, MyMove::Scissors(3)));
    }

    #[test]
    fn test_score() {
        let parsed_array = parse_test_array();
        let (o, m) = parsed_array[3];
        assert_eq!(score(&m, &o), 1);
    }

    #[test]
    fn test_parse_2() {
        let (m, o) = parse_2('A', 'X');
        assert_eq!(score(&o, &m), 3);
    }
}