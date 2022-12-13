use std::fs;

fn main() {
    println!("\n=== Day 3  ====");
    println!("Opening intput.txt");

    let inputs =
        fs::read_to_string("src/bin/day_3/input.txt").expect("Unable to find 'input.txt' !");
    //let inputs = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n\n".to_string();

    let mut total: i32 = 0;

    for x in inputs.split('\n') {
        if x.is_empty() {
            continue;
        }

        let (first, second) = x.split_at(x.len() / 2);
        let common = find_common(first, second);


        total += letter_to_priority(common) as i32;
    }

    println!("\nPart one answer: {}", total);
    // println!("\nPart two answer: {}", score_part_2);
}


fn find_common(a: &str, b: &str) -> char{

    for x in a.chars(){
        for y in b.chars() {
            if x == y {
                return x;
            }
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
