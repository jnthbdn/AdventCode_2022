use std::{
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

    let mut sum : u32 = 0;
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
        // println!("\tLeft : {:?}", left);
        // println!("\tRight : {:?}", right);

        if is_correct {
            sum += id;
        }

        id += 1;
    }

    println!("\nPart one answer: {}", sum);
    // println!("\nPart two answer: {}", map.find_way_from_possible_starts());
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
        }
        else if next_left.is_array() && next_right.is_array() {
            let result = is_orderred(&next_left.as_array(), &next_right.as_array());
            match result {
                Some(v) => return Some(v),
                None => {},
            }
        }
        else if next_left.is_number() && next_right.is_array() {
            let left_vec = vec![next_left.clone()];
            
            match is_orderred(&left_vec, &next_right.as_array()) {
                Some(v) => return Some(v),
                None => {},
            }
        }
        else if next_left.is_array() && next_right.is_number() {
            let right_vec = vec![next_right.clone()];

            match is_orderred(&next_left.as_array(), &right_vec) {
                Some(v) => return Some(v),
                None => {},
            }
        }
    }

    if left.peek().is_some() && right.peek().is_none() {
        return Some(false);
    }
    else if left.peek().is_none() && right.peek().is_some() {
        return Some(true);
    }

    return None;
}

#[derive(Debug, Clone)]
enum Element {
    Number(u32),
    Array(Vec<Element>),
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
