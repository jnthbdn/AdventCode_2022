use std::fs;

const ROCK : i8 = 1;
const PAPER : i8 = 2;
const SCISSORS : i8 = 3;

const WIN : i8 = 6;
const DRAW : i8 = 3;
const LOSE : i8 = 0;

fn main() {
    println!("\n=== Day 1  ====");
    println!("Opening intput.txt");

    let inputs = fs::read_to_string("src/bin/day_2/input.txt").expect("Unable to find 'input.txt' !");
    //let inputs = "A Y\nB X\nC Z\n\n".to_string();

    let mut score_part_1: i32 = 0;

    for x in inputs.split('\n'){
        if x.is_empty(){ continue; }

        let elf = letter_to_shape(x.chars().nth(0).unwrap()).expect(format!("Failed to parse line '{}'", x).as_str());
        let me = letter_to_shape(x.chars().nth(2).unwrap()).expect(format!("Failed to parse line '{}'", x).as_str());

        score_part_1 += (battle(me, elf) + me) as i32;
    }


    println!("\nPart one answer: {}", score_part_1);
}

fn letter_to_shape(letter: char) -> Result<i8, ()>{
    match letter {
        'A' | 'X' => Ok(ROCK),
        'B' | 'Y' => Ok(PAPER),
        'C' | 'Z' => Ok(SCISSORS), 
        _ => Err(())
    }
}

fn battle(me: i8, elf: i8) -> i8{
    if  me == ROCK && elf == SCISSORS ||
        me == SCISSORS && elf == PAPER ||
        me == PAPER && elf == ROCK {
            return WIN;
        }
    
    if me == elf { return DRAW; }

    return LOSE;
}
