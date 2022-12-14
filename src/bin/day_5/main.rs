use regex::Regex;
use std::{fs, collections::VecDeque};

fn main() {
    println!("\n=== Day 5  ====");

    const NB_STACKS: usize = 9;
    let inputs =
    fs::read_to_string("src/bin/day_5/input.txt").expect("Unable to find 'input.txt' !");
// "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n\n".to_string();

    let mut stack_result : String = String::default();
    let mut stacks: [VecDeque<char>; NB_STACKS] = Default::default();
    let move_reg = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Failed to create Regex");

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("[") {
            let bytes = line.as_bytes();

            for i in 0..NB_STACKS {
                if bytes[4 * i + 1] != ' ' as u8 {
                    stacks[i].push_front(bytes[4 * i + 1] as char);
                }
            }
        } else if line.starts_with("move") {
            let captures = move_reg.captures(line).expect("Failed to capture from Regex");

            let nb_crate: usize = captures[1].parse().unwrap();
            let from: usize = captures[2].parse::<usize>().unwrap() - 1;
            let to: usize = captures[3].parse::<usize>().unwrap() - 1;

            for _ in 0..nb_crate{
                let tmp = stacks[from].pop_back().unwrap();
                stacks[to].push_back( tmp );
            }

        }
    }

    for s in stacks{
        stack_result.push( s.back().unwrap().clone() );
    }

    println!("\nPart one answer: {}", stack_result);
    // println!("\nPart two answer: {}", overlap_total);
}
