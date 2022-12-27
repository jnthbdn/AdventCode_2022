use std::{cmp, fs};

use regex::Regex;

fn main() {
    println!("\n=== Day 9 ====");

    let inputs =
        fs::read_to_string("src/bin/day_9/input.txt").expect("Unable to find 'input.txt' !");
    // "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n\n".to_string();
    // "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20\n\n".to_string();

    let regex: Regex = Regex::new(r"^(.) ([0-9]+)$").expect("Failed to create Regex CMD");
    let mut fisrt_rope = Rope::new(1);
    let mut second_rope = Rope::new(9);

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        let captures = regex
            .captures(line)
            .expect(format!("Failed to parse '{}'", line).as_str());
        let direction = captures[1].to_string().chars().nth(0).unwrap();
        let iteration = captures[2].parse().unwrap();

        for _ in 0..iteration {
            match direction {
                'U' => {
                    fisrt_rope.move_up();
                    second_rope.move_up();
                }

                'D' => {
                    fisrt_rope.move_down();
                    second_rope.move_down();
                }

                'L' => {
                    fisrt_rope.move_left();
                    second_rope.move_left();
                }

                'R' => {
                    fisrt_rope.move_right();
                    second_rope.move_right();
                }

                _ => {
                    println!("Unknown '{}' direction.", direction);
                    break;
                }
            }

            fisrt_rope.update_tail();
            second_rope.update_tail();
        }
    }

    println!("\nPart one answer: {}", fisrt_rope.tail_history.len());
    println!("\nPart two answer: {}", second_rope.tail_history.len());
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

struct Rope {
    head: Point,
    knots: Vec<Point>,
    tail_history: Vec<Point>,
}

impl Rope {
    fn new(nb_knots: usize) -> Self {

        if nb_knots == 0 {
            panic!("the number of knots must be > 0");
        }

        Rope {
            head: Point::default(),
            knots: vec![Point::default(); nb_knots],
            tail_history: vec![Point::default()],
        }
    }

    fn move_up(&mut self) {
        self.head.y += 1;
    }

    fn move_down(&mut self) {
        self.head.y -= 1;
    }

    fn move_right(&mut self) {
        self.head.x += 1;
    }

    fn move_left(&mut self) {
        self.head.x -= 1;
    }

    fn update_tail(&mut self) {
        self.update_knot(0);

    }

    fn update_knot(&mut self, knot: usize) {
        if knot >= self.knots.len() {
            self.add_position_history();
            return;
        }

        let (sp1, sp2) = self.knots.split_at_mut(knot);

        let mut current: &mut Point = &mut sp2[0];
        let previous: &Point;

        if knot == 0 {
            previous = &self.head
        } else {
            previous = &sp1.last().unwrap()
        };

        let distance = Point {
            x: previous.x - current.x,
            y: previous.y - current.y,
        };

        if distance.x.abs() > 1 || distance.y.abs() > 1 {
            let x_sign = if distance.x.is_negative() { -1 } else { 1 };
            let y_sign = if distance.y.is_negative() { -1 } else { 1 };

            if distance.x.abs() > 1 && distance.y == 0 {
                current.x += 1 * x_sign;
            } else if distance.x == 0 && distance.y.abs() > 1 {
                current.y += 1 * y_sign;
            } else if distance.x.abs() >= 1 && distance.y.abs() >= 1 {
                current.x += 1 * x_sign;
                current.y += 1 * y_sign;
            } else {
                panic!("Unknown case !! ({:?})", distance);
            }

            self.update_knot(knot+1);
        }
    }

    fn add_position_history(&mut self) {
        if !self.tail_history.contains(&self.knots.last().unwrap()) {
            self.tail_history.push(self.knots.last().unwrap().clone());
        }
    }
}
