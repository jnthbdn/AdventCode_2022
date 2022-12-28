use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, fs, rc::Rc, str::Lines};

lazy_static! {
    static ref REGEX_ADDX: Regex =
        Regex::new(r"^addx (-?[0-9]+)$$").expect("Failed to create Regex ADDX");
}

fn main() {
    println!("\n=== Day 10 ====");

    let inputs =
        fs::read_to_string("src/bin/day_10/input.txt").expect("Unable to find 'input.txt' !");
        // fs::read_to_string("src/bin/day_10/example.txt").expect("Unable to find 'example.txt' !");
        // "noop\naddx 3\naddx -5\n\n".to_string();

    let mut lines = inputs.lines();

    let mut command: Box<dyn Command> = Box::new(StartCommand);

    let mut signal : i32 = 0;
    let mut cycle: i32 = 0;
    let x_register: Rc<RefCell<i32>> = Rc::new(RefCell::new(1));
    let mut crt : String = String::default();

    loop {
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            println!("cycle {}: {} (= {})", cycle, x_register.borrow(), cycle * (*x_register.borrow()));
            signal += cycle * (*x_register.borrow());
        }

        draw_pixel(&mut crt, &cycle, &(*x_register.borrow()));


        if command.is_finish() {
            let line = get_next_line(&mut lines);

            if line.is_none() {
                break;
            }

            let line = line.unwrap();

            if line.starts_with("noop") {
                command = Box::new(NoopCommand::new());
            } else if line.starts_with("addx") {
                let capt = REGEX_ADDX.captures(line).expect("Fail to capture ADDX command");
                command = Box::new(AddxCommand::new(capt[1].parse().unwrap(), Rc::clone(&x_register)));
            } else {
                panic!("Unknown operation '{}'", line);
            }
        }
        

        command.tick();
    }

    println!("\nPart one answer: {}", signal);
    println!("\nPart two answer: \n{}", crt);
}

fn get_next_line<'a>(lines: &'a mut Lines) -> Option<&'a str> {
    loop {
        match lines.next() {
            Some(l) => {
                if !l.is_empty() {
                    return Some(l);
                }
            }
            None => return None,
        }
    }
}

fn draw_pixel(crt: &mut String, cycle: &i32, x_register: &i32){

    let pos = (cycle - 1) % 40;

    if (x_register-1) <= pos && pos <= (x_register + 1) {
        crt.push('#');
    }
    else{
        crt.push(' ');
    }

    if cycle % 40 == 0 {
        crt.push('\n');
    }
}

trait Command {
    fn is_finish(&self) -> bool;
    fn tick(&mut self);
}

/* --- START COMMAND */

struct StartCommand;
impl Command for StartCommand {
    fn is_finish(&self) -> bool {
        true
    }

    fn tick(&mut self) {}
}

/* --- NOOP --- */

struct NoopCommand {
    done: bool,
}

impl NoopCommand {
    fn new() -> Self {
        NoopCommand { done: false }
    }
}

impl Command for NoopCommand {
    fn is_finish(&self) -> bool {
        self.done
    }

    fn tick(&mut self) {
        self.done = true;
    }
}

/* --- ADDX --- */

struct AddxCommand {
    tick_left: u8,
    add_value: i32,
    register: Rc<RefCell<i32>>,
}

impl AddxCommand {
    fn new(add_value: i32, register: Rc<RefCell<i32>>) -> Self {
        AddxCommand {
            tick_left: 2,
            add_value,
            register,
        }
    }
}

impl Command for AddxCommand {
    fn is_finish(&self) -> bool {
        self.tick_left == 0
    }

    fn tick(&mut self) {
        match self.tick_left {
            0 => {}
            1 => {
                self.tick_left = 0;
                self.register.replace_with(|&mut v| v + self.add_value);
            }
            _ => {
                self.tick_left -= 1;
            }
        }
    }
}
