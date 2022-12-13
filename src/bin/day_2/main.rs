use std::fs;

const ROCK: i8 = 1;
const PAPER: i8 = 2;
const SCISSORS: i8 = 3;

const WIN: i8 = 6;
const DRAW: i8 = 3;
const LOSE: i8 = 0;

fn main() {
    println!("\n=== Day 2  ====");

    let inputs =
        fs::read_to_string("src/bin/day_2/input.txt").expect("Unable to find 'input.txt' !");
    //let inputs = "A Y\nB X\nC Z\n\n".to_string();

    let mut score_part_1: i32 = 0;
    let mut score_part_2: i32 = 0;

    for x in inputs.split('\n') {
        if x.is_empty() {
            continue;
        }

        /* ===== PART ONE ===== */

        let elf = letter_to_shape(x.chars().nth(0).unwrap())
            .expect(format!("Failed to parse line '{}'", x).as_str());
        let me = letter_to_shape(x.chars().nth(2).unwrap())
            .expect(format!("Failed to parse line '{}'", x).as_str());

        score_part_1 += (battle(me, elf) + me) as i32;

        
        /* ===== PART ONE ===== */

        let endgame = letter_to_endgame(x.chars().nth(2).unwrap())
            .expect(format!("Failed to parse line '{}'", x).as_str());
        let me = match endgame {
            WIN => win_battle(elf),
            DRAW => elf,
            LOSE => lose_battle(elf),
            _ => 0,
        };

        score_part_2 += (endgame + me) as i32;
    }

    println!("\nPart one answer: {}", score_part_1);
    println!("\nPart two answer: {}", score_part_2);
}

fn letter_to_shape(letter: char) -> Result<i8, ()> {
    match letter {
        'A' | 'X' => Ok(ROCK),
        'B' | 'Y' => Ok(PAPER),
        'C' | 'Z' => Ok(SCISSORS),
        _ => Err(()),
    }
}

fn letter_to_endgame(letter: char) -> Result<i8, ()> {
    match letter {
        'X' => Ok(LOSE),
        'Y' => Ok(DRAW),
        'Z' => Ok(WIN),
        _ => Err(()),
    }
}

fn win_battle(shape: i8) -> i8 {
    match shape {
        ROCK => PAPER,
        PAPER => SCISSORS,
        SCISSORS => ROCK,
        _ => -1,
    }
}

fn lose_battle(shape: i8) -> i8 {
    match shape {
        ROCK => SCISSORS,
        PAPER => ROCK,
        SCISSORS => PAPER,
        _ => -1,
    }
}

fn battle(me: i8, elf: i8) -> i8 {
    if me == ROCK && elf == SCISSORS || me == SCISSORS && elf == PAPER || me == PAPER && elf == ROCK
    {
        return WIN;
    }

    if me == elf {
        return DRAW;
    }

    return LOSE;
}
