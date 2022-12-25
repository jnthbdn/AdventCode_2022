use std::{cmp, fs, ops::Range};

fn main() {
    println!("\n=== Day 8  ====");

    let inputs =
        fs::read_to_string("src/bin/day_8/input.txt").expect("Unable to find 'input.txt' !");
    // "30373\n25512\n65332\n33549\n35390\n\n".to_string();

    let mut trees = Trees::new();
    let mut visible_tree: u32 = 0;
    let mut best_scenic = 0;

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        trees.add_row(line);
    }

    for y in 0..trees.height {
        for x in 0..trees.width {
            if trees.is_tree_visible(x, y) {
                visible_tree += 1;
            }

            best_scenic = cmp::max(best_scenic, trees.tree_scenic_score(x, y));
        }
    }

    println!("\nPart one answer: {}", visible_tree);
    println!("\nPart two answer: {}", best_scenic);
}

struct Trees {
    trees: Vec<u8>,
    width: u32,
    height: u32,
}

impl Trees {
    fn new() -> Self {
        Trees {
            trees: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn add_row(&mut self, row: &str) {
        for c in row.chars() {
            self.trees
                .push(c.to_digit(10).expect("Unable to convert char !") as u8);
        }

        self.height += 1;

        if self.width == 0 {
            self.width = row.len() as u32;
        } else {
            self.width = cmp::min(self.width, row.len() as u32);
        }
    }

    fn get(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height {
            panic!(
                "Get coordinate is out of range ! (x: {}, y: {}, width: {}, height: {})",
                x, y, self.width, self.height
            );
        }

        let idx = (self.width * y + x) as usize;
        self.trees
            .get(idx)
            .expect(format!("Unable to find index '{}' (x: {}, y: {})", idx, x, y).as_str())
            .clone()
    }

    fn is_tree_visible(&self, x: u32, y: u32) -> bool {
        if x == 0 || y == 0 || x >= (self.width - 1) || y >= (self.height - 1) {
            return true;
        }

        let tree = self.get(x, y);
        
        return self.visible_from_x_range(tree, 0..x, y)
            || self.visible_from_x_range(tree, (x + 1)..self.width, y)
            || self.visible_from_y_range(tree, x, 0..y)
            || self.visible_from_y_range(tree, x, (y + 1)..self.height);
    }

    fn visible_from_x_range(&self, tree: u8, x_range: Range<u32>, y: u32) -> bool {
        for i in x_range {
            if self.get(i, y) >= tree {
                return false;
            }
        }

        return true;
    }

    fn visible_from_y_range(&self, tree: u8, x: u32, y_range: Range<u32>) -> bool {
        for i in y_range {
            if self.get(x, i) >= tree {
                return false;
            }
        }

        return true;
    }


    fn tree_scenic_score(&self, x: u32, y: u32) -> u32 {

        if x == 0 || y == 0 || x >= (self.width - 1) || y >= (self.height - 1) {
            return 0;
        }

        let mut score_top = 0;
        let mut score_bot = 0;
        let mut score_left = 0;
        let mut score_right = 0;

        let tree = self.get(x, y);

        for i in (0..y).rev() {
            score_top += 1;

            if self.get(x, i) >= tree {
                break;
            }
        }

        for i in (y+1)..self.height {
            score_bot += 1;

            if self.get(x, i) >= tree {
                break;
            }
        }

        for i in (0..x).rev() {
            score_left += 1;

            if self.get(i, y) >= tree {
                break;
            }
        }

        for i in (x+1)..self.width {
            score_right += 1;

            if self.get(i, y) >= tree {
                break;
            }
        }


        return score_top * score_bot * score_left * score_right;
    }
}
