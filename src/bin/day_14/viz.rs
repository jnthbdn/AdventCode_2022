use std::{time::{Duration, Instant}, io};

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal, Frame, layout::{Direction, Constraint, Layout, Rect, Alignment}, text::{Spans, Span}, widgets::{Paragraph, Wrap, Block, Borders, BorderType, Clear}, style::{Style, Modifier, Color}};

use crate::grid;
use crate::grid::Grid;


const CELL_WIDTH : u16 = 5;
const CELL_HEIGH : u16 = 3;

pub struct App<'a> {
    grid: &'a mut Grid,
    is_run: bool,
    step_per_sec: u64,

    is_grid_init: bool,
    offset_x: i32,
    offset_y: i32,
}

impl App<'_> {
    pub fn new<'a>(grid: &'a mut Grid) -> App {
        App {grid, is_run: false, step_per_sec: 20, is_grid_init: false, offset_x: 0, offset_y: 0}
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(1_000 / app.step_per_sec))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(());
                    }

                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        app.is_run = !app.is_run;
                    }

                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        if ! app.is_run {
                            app.grid.next_step();
                        }
                    }
                    
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        if ! app.is_run {
                            app.grid.next_sand();
                        }
                    }
                    
                    KeyCode::Char('+') => {
                        app.step_per_sec = std::cmp::min(app.step_per_sec + 2, 500)
                    }
                    
                    KeyCode::Char('-') => {
                        app.step_per_sec = std::cmp::max(app.step_per_sec - 2, 2)
                    }

                    KeyCode::Up => {
                        app.offset_y -= 1;
                    }

                    KeyCode::Down => {
                        app.offset_y += 1;
                    }

                    KeyCode::Left => {
                        app.offset_x -= 1;
                    }

                    KeyCode::Right => {
                        app.offset_x += 1;
                    }

                    _ => {}
                }
            }
        }

        if app.is_run && last_tick.elapsed() >= Duration::from_millis(1_000 / app.step_per_sec) {
            app.grid.next_step();
            last_tick = Instant::now();
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(f.size());

    let text = vec![
        Spans::from(vec![Span::styled("Simulation", Style::default().add_modifier(Modifier::UNDERLINED))]),
        Spans::from(format!("Sand generated: {}", app.grid.generated_sand())),
        Spans::from(format!("Steps: {}", app.grid.current_step())),
        Spans::from(format!("{} steps/second", app.step_per_sec)),
        Spans::from(""),
        Spans::from(vec![Span::styled("Commands", Style::default().add_modifier(Modifier::UNDERLINED))]),
        Spans::from("[N]ext step"),
        Spans::from("Next [S]and unit"),
        Spans::from( vec![  Span::styled("[P]lay", Style::default().add_modifier( if app.is_run {Modifier::REVERSED} else {Modifier::empty()})), 
                            Span::raw(" / "),
                            Span::styled("[P]ause", Style::default().add_modifier( if !app.is_run {Modifier::REVERSED} else {Modifier::empty()})) ] ),
        Spans::from("[+]/[-] Increase/Decrease speed (+/- 2 steps/s)"),
        Spans::from(""),
        Spans::from("⬆ ⬇ ⬅ ➡  Move view"),
        Spans::from(format!("Offset: ({}, {})", app.offset_x, app.offset_y)),
    ];

    let block = Block::default().borders(Borders::ALL).title("Informations");
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let block_view = Block::default().borders(Borders::ALL).title("Simulation View");
    render_grid(app, f, chunks[1]);

    f.render_widget(block_view, chunks[1]);

    if app.grid.is_over() {
        show_popup_over(f, chunks[1]);
    }

}

fn render_grid<B: Backend>(app: &mut App, f: &mut Frame<B>, area: Rect){

    let available_x: u16 = std::cmp::min(app.grid.width() as u16, area.width / CELL_WIDTH);
    let available_y: u16 = std::cmp::min(app.grid.height() as u16, area.height / CELL_HEIGH);

    let grid_width = CELL_WIDTH * available_x;
    let grid_height = CELL_HEIGH * available_y;
    let padding_h = std::cmp::max(0, (area.width as i32 - grid_width as i32) / 2) as u16;
    let padding_v = std::cmp::max(0, (area.height as i32 - grid_height as i32) / 2) as u16;

    let x_constraint: Vec<Constraint> = std::iter::repeat(Constraint::Length(CELL_WIDTH)).take(available_x as usize).collect();
    let y_constraint: Vec<Constraint> = std::iter::repeat(Constraint::Length(CELL_HEIGH)).take(available_y as usize).collect();

    app.offset_x = constraint(app.offset_x, 0, app.grid.width() - available_x as i32);
    app.offset_y = constraint(app.offset_y, 0, app.grid.height() - available_y as i32);

    if ! app.is_grid_init {

        app.offset_x = (app.grid.width() - available_x as i32) / 2;
        app.offset_y = (app.grid.height() - available_y as i32) / 2;

        app.is_grid_init = true;
    }

    let h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(padding_h),
            Constraint::Length(grid_width),
            Constraint::Min(padding_h),
        ])
        .split(area);

    let v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(padding_v),
            Constraint::Length(grid_height),
            Constraint::Min(padding_v)
        ])
        .split(h_layout[1]);

    let rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(x_constraint)
        .split(v_layout[1]);

    for x in 0..available_x{
        let cols = Layout::default()
            .direction(Direction::Vertical)
            .constraints(y_constraint.clone())
            .split(rows[x as usize]);

        for y in 0..available_y {

            let pos_x = x as i32 + app.offset_x;
            let pos_y = y as i32 + app.offset_y;

            let p_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default());

            let p = Paragraph::new(format!("{}", grid_type_to_string(&app.grid.get_cell(pos_x as i32, pos_y as i32))))
                .alignment(tui::layout::Alignment::Center)
                .block(p_block);

            f.render_widget(p, cols[y as usize]);
        }
    }

}

fn show_popup_over<B: Backend>(f: &mut Frame<B>, area: Rect){

    let text = "The simulation is over, the sand starts flow into the abyss below.";
    let width = std::cmp::min(100, ((text.len() + 4) as f32 / area.width as f32 * 100.0) as u16);
    let height = 3 + (width as f32 / 100.0) as u16;

    let padding_width = (100 - width as u16) / 2;
    let padding_heigt = (area.height - height) / 2;

    let block = Block::default()
    .title("  SIMULATION STATUS  ")
    .borders(Borders::ALL)
    .border_type(BorderType::Plain)
    .style(Style::default().bg(Color::White).fg(Color::Black));

    let content = Paragraph::new(text).alignment(Alignment::Center).block(block).wrap(Wrap{trim: true});

    let popup_layout_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(padding_heigt),
                Constraint::Max(height),
                Constraint::Max(padding_heigt),
            ]
        )
        .split(area);

    let popup_layout_h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(padding_width),
                Constraint::Percentage(width),
                Constraint::Percentage(padding_width),
            ]
        )
        .split(popup_layout_v[1]);

        f.render_widget(Clear, popup_layout_h[1]);
        f.render_widget(content, popup_layout_h[1]);
    
}

fn grid_type_to_string(t: &grid::Block) -> String{
    match t {
        grid::Block::AIR => String::from("  "),
        grid::Block::SAND => String::from("🔶"),
        grid::Block::ROCK => String::from("⬜"),
        _ => String::from("❌")
    }
}

fn constraint(value: i32, min: i32, max: i32) -> i32{
    std::cmp::max(min, std::cmp::min(value, max))
}
