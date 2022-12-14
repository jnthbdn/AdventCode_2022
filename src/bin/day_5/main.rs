use regex::Regex;
use std::{collections::VecDeque, fs};

fn main() {
    println!("\n=== Day 5  ====");

    const NB_STACKS: usize = 9;
    let inputs =
    fs::read_to_string("src/bin/day_5/input.txt").expect("Unable to find 'input.txt' !");
// "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n\n".to_string();

    let mut stacks_1: [VecDeque<char>; NB_STACKS] = Default::default();
    let mut stacks_2: [VecDeque<char>; NB_STACKS] = Default::default();

    let move_reg = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Failed to create Regex");

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("[") {
            let bytes = line.as_bytes();

            for i in 0..NB_STACKS {
                if bytes[4 * i + 1] != ' ' as u8 {
                    stacks_1[i].push_front(bytes[4 * i + 1] as char);
                    stacks_2[i].push_front(bytes[4 * i + 1] as char);
                }
            }
        } else if line.starts_with("move") {
            let captures = move_reg
                .captures(line)
                .expect("Failed to capture from Regex");

            let nb_crate: usize = captures[1].parse().unwrap();
            let from: usize = captures[2].parse::<usize>().unwrap() - 1;
            let to: usize = captures[3].parse::<usize>().unwrap() - 1;

            /* ===== FIRST PART ===== */
            for _ in 0..nb_crate {
                let tmp = stacks_1[from].pop_back().unwrap();
                stacks_1[to].push_back(tmp);
            }

            /* ===== SECOND PART ===== */
            let mut tmp = stacks_2[from].split_off(stacks_2[from].len() - nb_crate);
            stacks_2[to].append(&mut tmp);
        }
    }

    println!("\nPart one answer: {}", String::from_iter(stacks_1.iter().map(|x| x.back().unwrap())));
    println!("\nPart two answer: {}", String::from_iter(stacks_2.iter().map(|x| x.back().unwrap())));
}
