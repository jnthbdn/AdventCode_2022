use std::fs;

mod viz;
mod line;
mod grid;

use line::{Line, Point};
use grid::Grid;

use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};
use viz::{run_app, App};

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=== Day 14 ====");

    let inputs =
        fs::read_to_string("src/bin/day_14/input.txt").expect("Unable to find 'input.txt' !");
        // String::from("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n\n");

    let mut lines: Vec<Line> = Vec::new();

    for line in inputs.lines() {

        if line.is_empty() {
            continue;
        }

        let mut coords = line.split(" -> ");
        let mut last = extract_point( coords.next().expect(format!("No Coords on {}", line).as_str()) );

        while let Some(coord) = coords.next() {
            let next = extract_point(coord);

            lines.push(Line::new(last, next.clone()));
            last = next;
        }
    }

    let min_x = lines.iter().min_by(|a, b| a.start.x.cmp(&b.start.x)).unwrap().start.x;

    for line in &mut lines{
        line.translate(Point { x: -min_x, y: 0 });
    }

    let mut grid = Grid::new(lines, Point { x: 500-min_x, y: 0 });

    // println!("\nPart one answer: {}", sum_id_correct);
    // println!("\nPart two answer: {}", first_divider_pos * second_divider_pos);

    start_viz(&mut grid)
    // Ok(())
}

fn extract_point(coords: &str) -> Point {
    let mut pt = Point { x: 0, y: 0 };

    let mut splits = coords.split(",");

    pt.x = splits.next().expect(format!("Failed to extract x coord on {}", coords).as_str()).parse().expect(format!("Failed parse x coord on {}", coords).as_str());
    pt.y = splits.next().expect(format!("Failed to extract y coord on {}", coords).as_str()).parse().expect(format!("Failed parse y coord on {}", coords).as_str());

    pt
}

fn start_viz(grid: &mut Grid) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(grid);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
