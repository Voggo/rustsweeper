use crossterm::event::{DisableMouseCapture, EnableMouseCapture, MouseButton};
use rand::prelude::*;

use std::io::Write;
use std::io::{Stdout, stdout};

use anyhow::{Ok, Result};

use crossterm::terminal::{Clear, SetSize, size};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyModifiers},
    style::*,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
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
    // maybe add a draw position,
    // to both reference state and position at the same time
}

struct Board {
    grid: Vec<Vec<CellBox>>,
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize, mine_count: usize) -> Board {
        let mut grid = vec![
            vec![
                CellBox {
                    kind: CellKind::Number(1),
                    state: CellState::Hidden,
                };
                width
            ];
            height
        ];
        let mut rng = rand::rng();
        // TODO: fix rejection sampling to more efficient method
        for _i in 0..mine_count {
            let mut x: usize;
            let mut y: usize;
            loop {
                x = rng.random_range(0..width);
                y = rng.random_range(0..height);
                if let CellKind::Number(_) = grid[x][y].kind {
                    break;
                }
            }
            grid[x][y].kind = CellKind::Mine;
        }
        Board {
            grid,
            width,
            height,
        }
    }

    fn render_game_board(&self) {
        let mut stdout = stdout();
        stdout
            .execute(Clear(terminal::ClearType::All))
            .expect("Failed to clear line");
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        stdout
            .execute(MoveTo(board_start_x, board_start_y))
            .expect("Failed to move cursor to start position");
        let mut i = 0;
        for row in &self.grid {
            for cell in row {
                let (symbol, color) = match cell.state {
                    CellState::Hidden => ("■", Color::DarkGrey), // Hidden cell
                    CellState::Revealed => match cell.kind {
                        CellKind::Mine => ("💣", Color::Red), // Mine
                        CellKind::Number(n) => (
                            // Numbered cell
                            match n {
                                0 => " ",
                                1 => "1",
                                2 => "2",
                                3 => "3",
                                4 => "4",
                                5 => "5",
                                6 => "6",
                                7 => "7",
                                8 => "8",
                                _ => " ",
                            },
                            Color::Blue,
                        ),
                    },
                    CellState::Flagged => ("🚩", Color::Yellow), // Flagged cell
                };
                let symbol = if symbol == "💣" {
                    format!("{:>1}", symbol)
                } else {
                    format!("{:^2}", symbol)
                };

                // Print the symbol with the appropriate color
                stdout
                    .execute(SetForegroundColor(color))
                    .expect("Failed to set color");
                stdout
                    .execute(Print(symbol))
                    .expect("Failed to print symbol");
                // stdout.execute(ResetColor).expect("Failed to reset color");
            }
            i += 1;
            stdout
                .execute(MoveTo(0 + board_start_x, i + board_start_y))
                .expect("Failed to move cursor");
        }
    }
    fn get_board_start_pos(&self) -> (u16, u16) {
        let (cols, rows) = crossterm::terminal::size().expect("Failed to get terminal size");
        let board_start_x = (cols - (self.width * 2) as u16) / 2;
        let board_start_y = (rows - self.height as u16) / 2;
        (board_start_x, board_start_y)
    }
    fn handle_mouse_event(&mut self, event: event::MouseEvent) {
        if let event::MouseEventKind::Down(MouseButton::Left) = event.kind {
            let (board_start_x, board_start_y) = self.get_board_start_pos();
            let cell_x = (event.column as isize - board_start_x as isize) / 2;
            let cell_y = event.row as isize - board_start_y as isize;
            if cell_x >= self.width as isize || cell_y >= self.height as isize {
                write!(stdout(), "Mouse click out of bounds!").expect("!");
                return;
            } else {
                self.grid[cell_y][cell_x].state = match self.grid[cell_x][cell_y].state {
                    CellState::Hidden => CellState::Revealed, // Reveal the cell
                    CellState::Revealed => CellState::Flagged, // Flag the cell
                    CellState::Flagged => CellState::Hidden,  // Unflag the cell
                };
            }
        }
    }
}

// set up and clean up section
fn setup_terminal(mut stdout: &Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(EnableMouseCapture)?;
    return Ok(());
}

// set styles
fn set_styles(mut stdout: &Stdout) -> Result<()> {
    stdout.execute(SetBackgroundColor(Color::Yellow))?;
    stdout.execute(SetForegroundColor(Color::Red))?;

    // clean everything
    stdout.execute(Clear(terminal::ClearType::All))?;
    return Ok(());
}

fn cleanup_terminal(mut stdout: &Stdout) -> Result<()> {
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(DisableMouseCapture)?;
    terminal::disable_raw_mode()?;
    return Ok(());
}

fn should_exit(event: &Event) -> Result<bool> {
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

fn main() -> Result<()> {
    let mut stdout = stdout();
    // terminal set up
    setup_terminal(&stdout)?;
    set_styles(&stdout)?;
    let mut board = Board::new(10, 10, 8);
    board.render_game_board();

    'drawing: loop {
        let event = event::read()?;

        if let Event::Mouse(event) = event {
            // Handle mouse events here if needed
            // println!("Mouse event: {:?}", event);
            match event.kind {
                event::MouseEventKind::Moved => {
                    // Handle mouse movement if needed
                    stdout.execute(MoveTo(event.column, event.row))?;
                }
                event::MouseEventKind::Down(MouseButton::Left) => {
                    board.handle_mouse_event(event);
                    board.render_game_board();
                }
                _ => {
                    // Handle other mouse events (e.g., clicks, scrolls) if needed
                    // todo!("Handle other mouse events: {:?}", event.kind);
                }
            }
        }
        if should_exit(&event)? == true {
            break 'drawing;
        }
    }
    cleanup_terminal(&stdout)?;
    Ok(())
}
