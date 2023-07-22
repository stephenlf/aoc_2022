/// Key concepts:
/// * Idiomatic deserialization: TryFrom ??
/// * Production-ready deserialization: SERDE!!!
/// 

use std::fmt::Display;

#[derive(Debug)]
enum Item {
    Num(u32),
    List(List),
}

pub fn packets_are_ordered(left: &List, right: &List) -> Option<bool> {
    let len = left.0.len().min(right.0.len());
    for i in 0..len {
        match (&left.0[i], &right.0[i]) {
            (Item::Num(l), Item::Num(r)) => if l == r {continue} else {return Some(l < r)},
            (Item::List(l), Item::List(r)) => {
                let o = packets_are_ordered(&l, &r);
                if o.is_none() {continue} else {return o}
            },
            (Item::Num(l), Item::List(r)) => {
                let o = packets_are_ordered(&List(vec![Item::Num(*l)]), &r);
                if o.is_none() {continue} else {return o}
            },
            (Item::List(l), Item::Num(r)) => {
                let o = packets_are_ordered( &l, &List(vec![Item::Num(*r)]));
                if o.is_none() {continue} else {return o}
            },
        }
    }
    if left.0.len() == right.0.len() {None} else {Some(left.0.len() < right.0.len())}
}

#[derive(Debug)]
pub struct List(Vec<Item>);
impl List {
    pub fn new(s: String) -> Self {
        Self::new_list(&s[..]).0
    }
    
    fn new_list(s: &str) -> (Self, &str) {
        let mut v: Vec<Item> = Vec::new();

        let mut num: Option<u32> = None;
        let mut next_token = Token::OpenBracket;    //placeholder
        let mut s = s;

        loop {
            let result = Token::next_token(s);
            if result.is_err() {
                break;
            } else {
                (num, next_token, s) = result.unwrap();
            }
            if let Some(n) = num {
                v.push(Item::Num(n));
            }
            match next_token {
                Token::CloseBracket => break,
                Token::Comma => continue,
                Token::OpenBracket => {
                    let result = Self::new_list(s);
                    v.push(Item::List(result.0));
                    s = result.1;
                },
            }
        }

        (List(v), s)
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('[');
        for item in &self.0 {
            match item {
                Item::Num(n) => {
                    s.push_str(format!("{n}, ").as_str());
                }
                Item::List(l) => {
                    s.push_str(format!("{l}, ").as_str());
                }
            }
        }
        s.push(']');
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Comma,
}

impl Token {
    fn next_token(s: &str) -> Result<(Option<u32>, Token, &str), &'static str> {
        println!("next_token s: {s}");
        let result = s.find(|c| c == '[' || c == ']' || c == ',');
        let token_idx = if result.is_none() {
            return Err("No tokens left in string");
        } else {
            result.unwrap()
        };
        let token = match s.chars().nth(token_idx).unwrap() {
            '[' => Ok(Token::OpenBracket),
            ']' => Ok(Token::CloseBracket),
            ',' => Ok(Token::Comma),
            _ => {
                Err("Couldn't find valid token in given str")
            }
        }?;
        
        let num = if token_idx != 0 {
            let n = s[..token_idx].parse::<u32>().unwrap();
            Some(n)
        } else {
            None
        };

        Ok((num, token, s.get(token_idx + 1..).unwrap()))
    }
}

#[cfg(test)]
mod day_13 {
    use super::*;

    #[test]
    fn next_token() {
        let s = "[[0,1],[],3]";

        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == None && token == Token::OpenBracket && s == "[0,1],[],3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == None && token == Token::OpenBracket && s == "0,1],[],3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == Some(0) && token == Token::Comma && s == "1],[],3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == Some(1) && token == Token::CloseBracket && s == ",[],3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == None && token == Token::Comma && s == "[],3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == None && token == Token::OpenBracket && s == "],3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == None && token == Token::CloseBracket && s == ",3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == None && token == Token::Comma && s == "3]");
        
        let (num, token, s) = Token::next_token(&s).unwrap();
        assert!(num == Some(3) && token == Token::CloseBracket && s == "");

    }

    #[test]
    fn test_parser() {
        let s = "[[0,1],[],3]";
        let list = List::new_list(s);
        println!("{:?}", list);

        let s = "[[[]]]";
        let list = List::new_list(s);
        println!("{:?}", list);
    }

    #[test]
    fn test_comparitor() {
        let left = List::new("[[1],[2,3,4]]".to_string());
        let right = List::new("[[1],4]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[1,1,3,1,1]".to_string());
        let right = List::new("[1,1,5,1,1]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[9]".to_string());
        let right = List::new("[[8,7,6]]".to_string());
        assert!(!packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[1,1,3,1,1]".to_string());
        let right = List::new("[1,1,5,1,1]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[1,1,3,1,1]".to_string());
        let right = List::new("[1,1,5,1,1]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[1,1,3,1,1]".to_string());
        let right = List::new("[1,1,5,1,1]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[1,1,3,1,1]".to_string());
        let right = List::new("[1,1,5,1,1]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
        let left = List::new("[1,1,3,1,1]".to_string());
        let right = List::new("[1,1,5,1,1]".to_string());
        assert!(packets_are_ordered(&left, &right).unwrap());
 
    }
}