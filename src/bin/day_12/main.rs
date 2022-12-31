use std::fs;

fn main() {
    println!("\n=== Day 12 ====");

    let inputs =
        fs::read_to_string("src/bin/day_12/input.txt").expect("Unable to find 'input.txt' !");
    // "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n\n".to_string();

    let mut map = Map::new();

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        map.add_row(line);
    }

    println!("");

    println!("\nPart one answer: {}", map.find_way_from_start());
    println!("\nPart two answer: {}", map.find_way_from_possible_starts());
}

#[derive(Default, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist_from(&self, other: &Point) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
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

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Map {
    map: Vec<u32>,
    start: Point,
    end: Point,
    possible_starts: Vec<Point>,
    width: u32,
    height: u32,
}

impl Map {
    fn new() -> Self {
        Map {
            map: Vec::new(),
            start: Point::default(),
            end: Point::default(),
            possible_starts: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_row(&mut self, line: &str) {
        if self.width == 0 {
            self.width = line.len() as u32;
        } else if self.width != line.len() as u32 {
            panic!("The line have the wrong size !\n'{}'", line);
        }

        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    self.start.x = i as i32;
                    self.start.y = self.height as i32;
                    self.map.push(self.char_to_height_value('a'));
                    self.possible_starts.push(self.start.clone());
                }
                'E' => {
                    self.end.x = i as i32;
                    self.end.y = self.height as i32;
                    self.map.push(self.char_to_height_value('z'));
                }
                _ => {
                    if c == 'a' {
                        self.possible_starts.push(Point {
                            x: i as i32,
                            y: self.height as i32,
                        });
                    }

                    self.map.push(self.char_to_height_value(c));
                }
            }
        }

        self.height += 1;
    }

    fn find_way_from_start(&self) -> u32 {
        self.find_way(&vec![self.start.clone()])
    }

    fn find_way_from_possible_starts(&self) -> u32 {
        self.find_way(&self.possible_starts)
    }

    fn find_way(&self, starts: &Vec<Point>) -> u32 {
        let mut visited: Vec<Point> = starts.clone();
        let mut heads: Vec<Point> = starts.clone();
        let mut steps: u32 = 0;

        while !heads.contains(&self.end) {
            steps += 1;

            let old_heads = heads.clone();
            heads.clear();

            for head in old_heads {
                for dir in self.get_available_directions(&head, &visited) {
                    let next_head = match dir {
                        Direction::UP => Point {
                            x: head.x,
                            y: head.y - 1,
                        },
                        Direction::DOWN => Point {
                            x: head.x,
                            y: head.y + 1,
                        },
                        Direction::LEFT => Point {
                            x: head.x - 1,
                            y: head.y,
                        },
                        Direction::RIGHT => Point {
                            x: head.x + 1,
                            y: head.y,
                        },
                    };

                    heads.push(next_head.clone());
                    visited.push(next_head);
                }
            }
        }

        return steps;
    }

    fn can_go(&self, pos: &Point, dir: Direction, path: &Vec<Point>) -> bool {
        let current = self.get_height_value(pos).unwrap();

        let destination = match dir {
            Direction::UP => Point {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::DOWN => Point {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::LEFT => Point {
                x: pos.x - 1,
                y: pos.y,
            },
            Direction::RIGHT => Point {
                x: pos.x + 1,
                y: pos.y,
            },
        };

        let target = self.get_height_value(&destination);

        return match target {
            None => false,
            Some(t) => !path.contains(&destination) && t <= (current + 1),
        };
    }

    fn get_available_directions(&self, pos: &Point, path: &Vec<Point>) -> Vec<Direction> {
        let mut result: Vec<Direction> = Vec::new();

        if self.can_go(pos, Direction::UP, path) {
            result.push(Direction::UP);
        }

        if self.can_go(pos, Direction::DOWN, path) {
            result.push(Direction::DOWN);
        }

        if self.can_go(pos, Direction::LEFT, path) {
            result.push(Direction::LEFT);
        }

        if self.can_go(pos, Direction::RIGHT, path) {
            result.push(Direction::RIGHT);
        }

        return result;
    }

    fn get_height_value(&self, pos: &Point) -> Option<u32> {
        if pos.x >= self.width as i32 || pos.x < 0 || pos.y >= self.height as i32 || pos.y < 0 {
            return None;
        }

        let idx = pos.y * self.width as i32 + pos.x;
        return Some(self.map[idx as usize]);
    }

    fn char_to_height_value(&self, c: char) -> u32 {
        c as u32
    }
}
