use std::fs;

fn main() {
    println!("\n=== Day 3  ====");

    let inputs =
        fs::read_to_string("src/bin/day_3/input.txt").expect("Unable to find 'input.txt' !");
    //let inputs = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n\n".to_string();

    let mut total_priority: i32 = 0;
    let mut total_badges: i32 = 0;
    let mut pack: Vec<&str> = Vec::new();

    for x in inputs.split('\n') {
        if x.is_empty() {
            continue;
        }

        /* ===== FIRST PART ===== */

        let (first, second) = x.split_at(x.len() / 2);
        let common = find_common(first, second);

        total_priority += letter_to_priority(common) as i32;


        /* ===== SECOND PART ===== */

        pack.push(x);
        if pack.len() >= 3 {
            total_badges += letter_to_priority( find_badge(&pack) ) as i32;
            pack.clear();
        }
    }

    println!("\nPart one answer: {}", total_priority);
    println!("\nPart two answer: {}", total_badges);
}

fn find_badge(pack: &Vec<&str>) -> char{
    for c in pack.get(0).unwrap().chars(){
        if pack.get(1).unwrap().contains(c) && pack.get(2).unwrap().contains(c) {
            return c;
        }
    }

    return '\0';
}

fn find_common(a: &str, b: &str) -> char{

    for x in a.chars(){
        if b.contains(x) {
            return x;
        }
    }

    return '\0';
}

fn letter_to_priority(letter: char) -> i8{

    let mut result : i8 = 0;

    if letter.is_uppercase() {
        result = 26;
    }

    result += (letter.to_ascii_lowercase() as i8) - 96;

    return result;
}
