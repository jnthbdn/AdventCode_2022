use core::fmt;
use std::{
    cmp::Ordering,
    fs,
    iter::Peekable,
    slice::Iter,
    str::{Chars, Lines},
};

fn main() {
    println!("\n=== Day 13 ====");

    let inputs =
        fs::read_to_string("src/bin/day_13/input.txt").expect("Unable to find 'input.txt' !");
    // fs::read_to_string("src/bin/day_13/example.txt").expect("Unable to find 'input.txt' !");

    let mut lines = inputs.lines();

    let mut packets: Vec<Vec<Element>> = Vec::new();
    let mut sum_id_correct: u32 = 0;
    let mut id: u32 = 1;
    loop {
        let line_left = get_next_line(&mut lines);
        let line_right = get_next_line(&mut lines);
        if line_left.is_none() || line_right.is_none() {
            break;
        }

        let left = parse_packet(&line_left.unwrap());
        let right = parse_packet(&line_right.unwrap());

        let is_correct = match is_orderred(&left, &right) {
            Some(v) => v,
            None => false,
        };

        // println!("\n=== Packet {} ===", id);
        // println!("\tis correct: {}", is_correct);
        // println!("\tLeft : {}", Element::Array(left.clone()));
        // println!("\tRight : {}", Element::Array(right.clone()));

        if is_correct {
            sum_id_correct += id;

            packets.push(left);
            packets.push(right);
        } else {
            packets.push(right);
            packets.push(left);
        }

        id += 1;
    }

    // ==== SORT & DIVIDER STUFF ====
    let first_divider = Element::Array(vec!(Element::Number(2)));
    let second_divider = Element::Array(vec!(Element::Number(6)));

    packets.push(vec!(first_divider.clone()));
    packets.push(vec!(second_divider.clone()));

    packets.sort_by(|a, b| match is_orderred(a, b) {
        Some(v) => match v {
            true => Ordering::Less,
            false => Ordering::Greater,
        },
        None => Ordering::Equal,
    });

    let first_divider_pos = 1 + packets.iter().position(|p| p.len() > 0 && p[0] == first_divider).unwrap();
    let second_divider_pos = 1 + packets.iter().position(|p| p.len() > 0 && p[0] == second_divider).unwrap();

    // println!("========== AFTER ==========\n");
    // for e in packets.iter() {
    //     println!("{}", Element::Array(e.clone()));
    // }


    println!("\nPart one answer: {}", sum_id_correct);
    println!("\nPart two answer: {}", first_divider_pos * second_divider_pos);
}

fn get_next_line(lines: &mut Lines) -> Option<String> {
    loop {
        match lines.next() {
            Some(l) => {
                if !l.is_empty() {
                    return Some(String::from(l));
                }
            }
            None => return None,
        }
    }
}

fn parse_packet(str: &String) -> Vec<Element> {
    let mut chars: Peekable<Chars> = str.chars().peekable();
    chars.next();

    return parse_array(&mut chars);
}

fn parse_array(chars: &mut Peekable<Chars>) -> Vec<Element> {
    let mut result: Vec<Element> = Vec::new();

    while chars.peek().is_some() {
        match chars.peek().unwrap() {
            '[' => {
                chars.next();
                result.push(Element::Array(parse_array(chars)));
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                result.push(Element::Number(parse_integer(chars)));
            }
            _ => {
                if chars.next().unwrap() == ']' {
                    break;
                }
            }
        }
    }

    return result;
}

fn parse_integer(chars: &mut Peekable<Chars>) -> u32 {
    let mut result: u32 = 0;

    while chars.peek().is_some() && chars.peek().unwrap().to_digit(10).is_some() {
        result = result * 10 + chars.next().unwrap().to_digit(10).unwrap();
    }

    return result;
}

fn is_orderred(left: &Vec<Element>, right: &Vec<Element>) -> Option<bool> {
    let mut left: Peekable<Iter<Element>> = left.iter().peekable();
    let mut right: Peekable<Iter<Element>> = right.iter().peekable();

    while left.peek().is_some() && right.peek().is_some() {
        let next_left = left.next().unwrap();
        let next_right = right.next().unwrap();

        if next_left.is_number() && next_right.is_number() {
            if next_left.as_number() == next_right.as_number() {
                continue;
            }

            return Some(next_left.as_number() < next_right.as_number());
        } else if next_left.is_array() && next_right.is_array() {
            let result = is_orderred(&next_left.as_array(), &next_right.as_array());
            match result {
                Some(v) => return Some(v),
                None => {}
            }
        } else if next_left.is_number() && next_right.is_array() {
            let left_vec = vec![next_left.clone()];

            match is_orderred(&left_vec, &next_right.as_array()) {
                Some(v) => return Some(v),
                None => {}
            }
        } else if next_left.is_array() && next_right.is_number() {
            let right_vec = vec![next_right.clone()];

            match is_orderred(&next_left.as_array(), &right_vec) {
                Some(v) => return Some(v),
                None => {}
            }
        }
    }

    if left.peek().is_some() && right.peek().is_none() {
        return Some(false);
    } else if left.peek().is_none() && right.peek().is_some() {
        return Some(true);
    }

    return None;
}

#[derive(Debug, Clone)]
enum Element {
    Number(u32),
    Array(Vec<Element>),
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if (self.is_array() && other.is_number()) || (self.is_number() && other.is_array()) {
            return false;
        }

        match &self {
            Element::Number(v) => v == &other.as_number(),
            Element::Array(a) => a == &other.as_array()
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Array(a) => {
                write!(f, "[")?;
                for (i, e) in a.iter().enumerate() {
                    if i == a.len() -1 {
                        write!(f, "{}", e)?;
                    }
                    else{
                        write!(f, "{}, ", e)?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Element {
    fn is_number(&self) -> bool {
        match self {
            Element::Number(_) => true,
            _ => false,
        }
    }

    fn is_array(&self) -> bool {
        match self {
            Element::Array(_) => true,
            _ => false,
        }
    }

    fn as_number(&self) -> u32 {
        match self {
            Element::Number(a) => a.clone(),
            _ => panic!("Not a Number !"),
        }
    }

    fn as_array(&self) -> Vec<Element> {
        match self {
            Element::Array(a) => a.clone(),
            _ => panic!("Not an Array !"),
        }
    }
}
