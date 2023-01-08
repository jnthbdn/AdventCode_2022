use std::{vec::IntoIter};


#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        let start: Point;
        let end : Point;

        if (a.x == b.x && a.y <= b.y) || (a.y == b.y && a.x <= b.x){
            start = a;
            end = b;
        }
        else{
            start = b;
            end = a;
        }

        Line {start, end}
    }

    pub fn translate(&mut self, t: Point){

        self.start.x += t.x;
        self.end.x += t.x;

        self.start.y += t.y;
        self.end.y += t.y;
    }

    pub fn to_iter(&self) -> IntoIter<Point>{
        let delta_x = self.end.x - self.start.x;
        let delta_y = self.end.y - self.start.y;
        let mut result: Vec<Point> = Vec::new();

        if delta_x >= delta_y {
            for x in self.start.x..(self.end.x + 1) {
                let new_y = self.start.y + delta_y * (x - self.start.x) / delta_x;
                result.push(Point { x: x, y: new_y });
            }
        }
        else{
            for y in self.start.y..(self.end.y + 1) {
                let new_x = self.start.x + delta_x * (y - self.start.y) / delta_y;
                result.push(Point { x: new_x, y });
            }
        }

        result.into_iter()
    }
}
