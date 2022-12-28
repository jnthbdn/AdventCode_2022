use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, fs, str::Lines};

lazy_static! {
    static ref REG_ITEM: Regex =
        Regex::new(r"Starting items: ([0-9, ]+)").expect("Failed to create REG_ITEM");
    static ref REG_OPER: Regex = Regex::new(r"Operation: new = old ([\+\*]) ([0-9]+|old)")
        .expect("Failed to create REG_OPER");
    static ref REG_TEST: Regex =
        Regex::new(r"Test: divisible by ([0-9]+)").expect("Failed to create REG_TEST");
    static ref REG_TRUE: Regex =
        Regex::new(r"If true: throw to monkey ([0-9]+)").expect("Failed to create REG_TRUE");
    static ref REG_FALSE: Regex =
        Regex::new(r"If false: throw to monkey ([0-9]+)").expect("Failed to create REG_FALSE");
}

fn main() {
    println!("\n=== Day 11 ====");

    let inputs =
        fs::read_to_string("src/bin/day_11/input.txt").expect("Unable to find 'input.txt' !");
    // fs::read_to_string("src/bin/day_11/example.txt").expect("Unable to find 'example.txt' !");

    let mut lines = inputs.lines();
    let mut monkeys_first: Vec<Monkey> = Vec::new();
    let mut monkeys_second: Vec<Monkey> = Vec::new();
    let mut factor: u32 = 1;

    loop {
        let line = lines.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if !line.starts_with("Monkey") {
            continue;
        }

        let monkey = parse_monkey(&mut lines);
        factor *= monkey.div_test;

        monkeys_first.push(monkey.clone());
        monkeys_second.push(monkey);
    }
    println!("");

    println!(
        "\nPart one answer: {}",
        play_turns(20, &mut monkeys_first, |x| x / 3)
    );
    println!(
        "\nPart two answer: {}",
        play_turns(10_000, &mut monkeys_second, |x| x % (factor as u64))
    );
}

fn play_turns<F>(nb_turns: u32, monkeys: &mut Vec<Monkey>, mut relief: F) -> u64
where
    F: FnMut(u64) -> u64,
{
    let mut most_active_1: u64 = 0;
    let mut most_active_2: u64 = 0;

    for _ in 0..nb_turns {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                (&mut monkeys[i]).inspect(&mut relief);
                let (item, target) = (&mut monkeys[i]).throw().unwrap();

                (&mut monkeys[target as usize]).items.push_back(item);
            }
        }
    }

    println!("\nMonkeys inspections after {} turns:", nb_turns);
    for i in 0..monkeys.len() {
        let nb_inspection = monkeys[i].inspection_count;

        if most_active_1 < nb_inspection {
            most_active_2 = most_active_1;
            most_active_1 = nb_inspection;
        } else if most_active_2 < nb_inspection {
            most_active_2 = nb_inspection;
        }

        println!("\tMonkey {} inspected items {} times", i, nb_inspection);
    }

    return most_active_1 * most_active_2;
}

fn parse_monkey(lines: &mut Lines) -> Monkey {
    let capt_items = REG_ITEM.captures(lines.next().unwrap()).unwrap();
    let capt_operation = REG_OPER.captures(lines.next().unwrap()).unwrap();
    let capt_test = REG_TEST.captures(lines.next().unwrap()).unwrap();
    let capt_true = REG_TRUE.captures(lines.next().unwrap()).unwrap();
    let capt_false = REG_FALSE.captures(lines.next().unwrap()).unwrap();

    let operation: Operation;

    if capt_operation[1].contains("+") {
        operation = Operation::ADD(capt_operation[2].parse().unwrap());
    } else if capt_operation[1].contains("*") {
        if capt_operation[2].contains("old") {
            operation = Operation::SQR();
        } else {
            operation = Operation::MUL(capt_operation[2].parse().unwrap());
        }
    } else {
        panic!("Unknown operation '{}'", capt_operation[1].to_string());
    }

    return Monkey {
        items: capt_items[1]
            .split(", ")
            .map(|e| e.parse().unwrap())
            .collect(),
        operation,
        div_test: capt_test[1].parse().unwrap(),
        test_true_monkey: capt_true[1].parse().unwrap(),
        test_false_monkey: capt_false[1].parse().unwrap(),
        inspection_count: 0,
    };
}

#[derive(Debug, Clone)]
enum Operation {
    ADD(u64),
    MUL(u64),
    SQR(),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    div_test: u32,
    test_true_monkey: u32,
    test_false_monkey: u32,

    inspection_count: u64,
}

impl Monkey {
    fn inspect<F>(&mut self, mut relief: F)
    where
        F: FnMut(u64) -> u64,
    {
        let item = self.items.front_mut().unwrap();

        match self.operation {
            Operation::ADD(value) => {
                *item += value;
            }
            Operation::MUL(value) => {
                *item *= value;
            }
            Operation::SQR() => {
                *item *= *item;
            }
        };

        *item = relief(*item);
        self.inspection_count += 1;
    }

    /**
     * first tuple element: the worry level
     * second tuple element: the target monkey
     */
    fn throw(&mut self) -> Option<(u64, u32)> {
        if self.items.is_empty() {
            return None;
        }

        let item = self.items.pop_front().unwrap();

        if item % self.div_test as u64 == 0 {
            return Some((item, self.test_true_monkey));
        } else {
            return Some((item, self.test_false_monkey));
        }
    }

    fn clone(&self) -> Monkey {
        Monkey {
            items: self.items.clone(),
            operation: self.operation.clone(),
            div_test: self.div_test,
            test_true_monkey: self.test_true_monkey,
            test_false_monkey: self.test_false_monkey,
            inspection_count: self.inspection_count,
        }
    }
}
