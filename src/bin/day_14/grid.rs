use crate::line::{Line, Point};

#[derive(Clone, PartialEq)]
pub enum Block {
    AIR,
    ROCK,
    SAND,
    OUT,
}

#[derive(Clone)]
pub struct Grid {
    grid: Vec<Vec<Block>>,
    width: i32,
    height: i32,
    sand_input: Point,
    step: u32,
    generated_sand: u32,
    current_sand: Option<Point>,
    is_over: bool,
}

impl Grid {
    pub fn new(rocks: Vec<Line>, sand_input: Point) -> Self {
        let x_max = rocks
            .iter()
            .max_by(|a, b| a.end.x.cmp(&b.end.x))
            .unwrap()
            .end
            .x;
        let y_max = rocks
            .iter()
            .max_by(|a, b| a.end.y.cmp(&b.end.y))
            .unwrap()
            .end
            .y;
        let mut grid: Vec<Vec<Block>> = Vec::new();

            println!("Grid size {}x{}", x_max+1, y_max+1);

        for _ in 0..(y_max+1) {
            let mut tmp: Vec<Block> = Vec::new();
            for _ in 0..(x_max+1) {
                tmp.push(Block::AIR);
            }
            grid.push(tmp);
        }

        for line in rocks {
            for pos in line.to_iter() {
                grid[pos.y as usize][pos.x as usize] = Block::ROCK;
            }
        }

        Grid {
            grid,
            width: x_max+1,
            height: y_max+1,
            sand_input,
            step: 0,
            generated_sand: 0,
            current_sand: None,
            is_over: false
        }
    }

    fn set_cell(&mut self, point: &Point, t: Block) {
        self.grid[point.y as usize][point.x as usize] = t;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Block {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            Block::OUT
        }
        else{
            self.grid[y as usize][x as usize].clone()
        }
    }

    fn next_available_block(&self, x: i32, y: i32) -> Option<Point>{

        match self.get_cell(x, y + 1) {
            Block::AIR | Block::OUT => return Some(Point{x, y: y+1}),
            _ => {}
        }
        match self.get_cell(x - 1, y + 1) {
            Block::AIR | Block::OUT => return Some(Point{x: x - 1, y: y+1}),
            _ => {}
        }
        match self.get_cell(x + 1, y + 1) {
            Block::AIR | Block::OUT => return Some(Point{x: x + 1, y: y+1}),
            _ => {}
        }

        return None;
    }

    pub fn next_step(&mut self) {
        let next_pos: Point;

        if self.is_over {
            return;
        }

        self.step += 1;

        match &self.current_sand {
            None => {
                next_pos = self.sand_input.clone();
                self.generated_sand += 1;
            }
            Some(sand) => {
                match self.next_available_block(sand.x, sand.y) {
                    Some(pos) => {
                        if self.get_cell(pos.x, pos.y) == Block::OUT {
                            self.current_sand = None;
                            self.is_over = true;
                            return;
                        }
                        else{
                            next_pos = pos;
                        }
                    },
                    None => {
                        self.current_sand = None;
                        return;
                    }
                }

                let old = &self.current_sand.as_ref().unwrap().clone();
                self.set_cell(old, Block::AIR);
            }
        }

        self.set_cell(&next_pos, Block::SAND);
        self.current_sand = Some(next_pos);

    }

    pub fn next_sand(&mut self){
        loop {
            self.next_step();

            if self.current_sand.is_none(){
                break;
            }
        }
    }

    pub fn current_step(&self) -> u32 {
        self.step.clone()
    }

    pub fn generated_sand(&self) -> u32 {
        self.generated_sand.clone()
    }

    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn is_over(&self) -> bool { self.is_over }
}
