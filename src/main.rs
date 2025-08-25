use crossterm::event::{DisableMouseCapture, EnableMouseCapture, MouseButton};
use rand::prelude::*;

use std::io::Write;
use std::io::{Stdout, stdout};

// use anyhow::{Ok, Result};

use crossterm::terminal::{Clear, SetSize, size};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    style::*,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    {execute, queue},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellKind {
    Mine,
    Number(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CellBox {
    kind: CellKind,
    state: CellState,
}

struct Board {
    grid: Vec<CellBox>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let mut grid = vec![
            CellBox {
                kind: CellKind::Number(0),
                state: CellState::Hidden,
            };
            width * height
        ];
        Board {
            grid,
            width,
            height,
        }
    }

    fn place_mines(&mut self, mine_count: usize) {
        let mut rng = rand::rng();
        let mut set_index = (0..(self.width * self.height)).collect::<Vec<usize>>();
        set_index.shuffle(&mut rng);
        for i in 0..mine_count {
            let idx = set_index[i];
            self.grid[idx].kind = CellKind::Mine;
            let x = (idx % self.width) as isize;
            let y = (idx as isize - x) / self.width as isize;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if let Some(neighbor) = self.get_cell_mut(x + dx, y + dy) {
                        if let CellKind::Number(ref mut n) = neighbor.kind {
                            *n += 1;
                        }
                    }
                }
            }
        }
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<&CellBox> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y * self.width as isize + x;
            Some(&self.grid[index as usize])
        } else {
            None
        }
    }

    fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut CellBox> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y * self.width as isize + x;
            Some(&mut self.grid[index as usize])
        } else {
            None
        }
    }

    fn render_game_board(&self, stdout: &mut Stdout) -> anyhow::Result<()> {
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        queue!(
            stdout,
            Clear(terminal::ClearType::All),
            MoveTo(board_start_x, board_start_y)
        )?;
        let mut i = 0;
        for y in 0..self.width {
            for x in 0..self.height as isize {
                if let Some(cell) = self.get_cell(x as isize, y as isize) {
                    let (symbol, color) = match cell.state {
                        CellState::Hidden => ("■", Color::DarkGrey), // Hidden cell
                        CellState::Revealed => match cell.kind {
                            CellKind::Mine => ("💣", Color::Black),      // Mine
                            CellKind::Number(0) => ("  ", Color::Black), // Empty cell
                            CellKind::Number(n) => (&n.to_string()[..], Color::Blue),
                        },
                        CellState::Flagged => ("F", Color::DarkRed), // Flagged cell
                    };
                    let symbol = if symbol == "💣" {
                        format!("{:>1}", symbol)
                    } else {
                        format!("{:^2}", symbol)
                    };
                    queue!(stdout, SetForegroundColor(color), Print(symbol))?;
                }
            }
            i += 1;
            queue!(stdout, MoveTo(board_start_x, i + board_start_y))?;
        }
        stdout.flush()?;
        Ok(())
    }
    fn get_board_start_pos(&self) -> (u16, u16) {
        let (cols, rows) = crossterm::terminal::size().expect("Failed to get terminal size");
        let board_start_x = (cols as i16 - (self.width * 2) as i16) / 2;
        let board_start_y = (rows as i16 - self.height as i16) / 2;
        (board_start_x as u16, board_start_y as u16)
    }
    fn handle_mouse_left(&mut self, event: event::MouseEvent) {
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        let cell_x = (event.column as isize - board_start_x as isize) / 2;
        let cell_y = event.row as isize - board_start_y as isize;
        if let Some(cell) = self.get_cell_mut(cell_x, cell_y) {
            cell.state = match cell.state {
                CellState::Hidden => CellState::Revealed,
                _ => cell.state, // Do nothing if it's already revealed or flagged
            };
        } else {
            write!(stdout(), "Mouse click out of bounds!").expect("!");
        }
    }
    fn handle_mouse_right(&mut self, event: event::MouseEvent) {
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        let cell_x = (event.column as isize - board_start_x as isize) / 2;
        let cell_y = event.row as isize - board_start_y as isize;
        if let Some(cell) = self.get_cell_mut(cell_x, cell_y) {
            cell.state = match cell.state {
                CellState::Hidden => CellState::Flagged,
                CellState::Flagged => CellState::Hidden,
                _ => cell.state, // Do nothing if it's already revealed
            };
        } else {
            write!(stdout(), "Mouse click out of bounds!").expect("!");
        }
    }
}

// set up and clean up section
fn setup_terminal(mut stdout: &Stdout) -> Result<(), std::io::Error> {
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    return Ok(());
}

// set styles
fn set_styles(mut stdout: &Stdout) -> Result<(), std::io::Error> {
    execute!(
        stdout,
        SetBackgroundColor(Color::Yellow),
        SetForegroundColor(Color::Blue),
        Clear(terminal::ClearType::All)
    )?;
    return Ok(());
}

fn cleanup_terminal(mut stdout: &Stdout) -> Result<(), std::io::Error> {
    execute!(stdout, Show, LeaveAlternateScreen, DisableMouseCapture)?;
    terminal::disable_raw_mode()?;
    return Ok(());
}

fn should_exit(event: &Event) -> Result<bool, std::io::Error> {
    if let Event::Key(key_event) = event.to_owned() {
        if key_event.code == KeyCode::Esc
            || (key_event.code == KeyCode::Char('c')
                && key_event.modifiers == KeyModifiers::CONTROL)
        {
            return Ok(true);
        }
    }
    return Ok(false);
}

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = stdout();
    // terminal set up
    setup_terminal(&stdout)?;
    set_styles(&stdout)?;
    let mut board = Board::new(10, 10);
    board.place_mines(8);
    board.render_game_board(&mut stdout)?;

    'drawing: loop {
        let event = event::read()?;
        if should_exit(&event)? == true {
            break 'drawing;
        }

        if let Event::Mouse(event) = event {
            // Handle mouse events here if needed
            // println!("Mouse event: {:?}", event);
            match event.kind {
                event::MouseEventKind::Moved => {
                    // Handle mouse movement if needed
                    stdout.execute(MoveTo(event.column, event.row))?;
                }
                event::MouseEventKind::Down(MouseButton::Left) => {
                    board.handle_mouse_left(event);
                }

                event::MouseEventKind::Down(MouseButton::Right) => {
                    board.handle_mouse_right(event);
                }
                _ => {
                    // Handle other mouse events (e.g., clicks, scrolls) if needed
                    // todo!("Handle other mouse events: {:?}", event.kind);
                }
            }
        }

        board.render_game_board(&mut stdout)?;
    }
    cleanup_terminal(&stdout)?;
    Ok(())
}
