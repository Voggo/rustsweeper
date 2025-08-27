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

enum GameState {
    New,
    Ongoing,
    Won,
    Lost,
}

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
    mines_placed: bool,
    mines_to_place: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        let grid = vec![
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
            mines_placed: false,
            mines_to_place: 99,
        }
    }

    fn place_mines(&mut self, x_zero: isize, y_zero: isize) {
        let mut rng = rand::rng();
        let mut set_index = (0..(self.width * self.height)).collect::<Vec<usize>>();
        set_index.shuffle(&mut rng);
        for i in 0..self.mines_to_place {
            let mut idx = set_index[i];
            if (idx % self.width) as isize == x_zero
                && (idx as isize - x_zero) / self.width as isize == y_zero
            {
                // if the random index is the same as the first clicked cell
                // pick the next index in the shuffled list this will always be valid
                idx = set_index[self.mines_to_place];
            }
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
    fn get_board_start_pos(&self) -> (u16, u16) {
        let (cols, rows) = crossterm::terminal::size().expect("Failed to get terminal size");
        let board_start_x = (cols as i16 - (self.width * 2) as i16) / 2;
        let board_start_y = (rows as i16 - self.height as i16) / 2;
        (board_start_x as u16, board_start_y as u16)
    }
    fn reveal_adjacent_empty(&mut self, x: isize, y: isize) -> Option<GameState> {
        let mut to_reveal = vec![(x, y)];
        while let Some((cx, cy)) = to_reveal.pop() {
            if let Some(cell) = self.get_cell_mut(cx, cy) {
                match cell.state {
                    CellState::Revealed | CellState::Flagged => continue, // Skip if already revealed or flagged
                    CellState::Hidden => {
                        cell.state = CellState::Revealed;
                        if cell.kind == CellKind::Mine {
                            return Some(GameState::Lost); // Stop if it's a mine
                        }
                    }
                }
                if cell.kind != CellKind::Number(0) {
                    continue; // Stop if it's not an empty cell
                }
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if let Some(neighbor) = self.get_cell_mut(cx + dx, cy + dy) {
                            match neighbor.kind {
                                CellKind::Number(_) => {
                                    to_reveal.push((cx + dx, cy + dy));
                                }
                                _ => (), // Do nothing for mines
                            }
                        }
                    }
                }
            }
        }
        return None;
    }
    fn reveal_non_flagged(&mut self, x: isize, y: isize) -> Option<GameState> {
        let mut ret = None;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(neighbor) = self.get_cell_mut(x + dx, y + dy) {
                    if neighbor.state != CellState::Flagged {
                        neighbor.state = CellState::Revealed;
                        if neighbor.kind == CellKind::Mine {
                            ret = Some(GameState::Lost);
                        }
                    }
                }
            }
        }
        return ret;
    }
    fn handle_mouse_left(&mut self, event: event::MouseEvent) -> Option<GameState> {
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        let cell_x = (event.column as isize - board_start_x as isize) / 2;
        let cell_y = event.row as isize - board_start_y as isize;
        if self.mines_placed == false {
            self.place_mines(cell_x, cell_y);
            self.mines_placed = true;
        }
        if let Some(cell) = self.get_cell_mut(cell_x, cell_y) {
            match cell.state {
                CellState::Hidden => {
                    // Only reveal if the cell is hidden
                    if let Some(game_state_change) = self.reveal_adjacent_empty(cell_x, cell_y) {
                        return Some(game_state_change);
                    };
                }
                CellState::Revealed => {
                    // If already revealed, reveal adjacent non-flagged cells if it's a number
                    if let CellKind::Number(n) = cell.kind {
                        if n > 0 {
                            if let Some(game_state_change) = self.reveal_non_flagged(cell_x, cell_y)
                            {
                                return Some(game_state_change);
                            };
                        }
                    }
                }
                _ => (), // Do nothing if it's already revealed or flagged
            };
        } else {
            write!(stdout(), "Mouse click out of bounds!").expect("!");
        }
        None
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

fn render_game_board(board: &Board, stdout: &mut Stdout) -> anyhow::Result<()> {
    let (board_start_x, board_start_y) = board.get_board_start_pos();
    queue!(
        stdout,
        Clear(terminal::ClearType::All),
        MoveTo(board_start_x, board_start_y)
    )?;
    let mut i = 0;
    for y in 0..board.width {
        for x in 0..board.height as isize {
            if let Some(cell) = board.get_cell(x as isize, y as isize) {
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
    render_game_board(&board, &mut stdout)?;
    let mut game_state = GameState::Ongoing;

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
                    if let Some(game_state_change) = board.handle_mouse_left(event) {
                        game_state = game_state_change;
                    };
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

        if let GameState::Lost = game_state {
            // Reveal all mines
            for cell in board.grid.iter_mut() {
                if cell.kind == CellKind::Mine {
                    cell.state = CellState::Revealed;
                }
            }
            render_game_board(&board, &mut stdout)?;
            cleanup_terminal(&stdout)?;
            println!("You hit a mine! Game Over!");
            std::thread::sleep(std::time::Duration::from_secs(3));
            return Ok(());
        }

        render_game_board(&board, &mut stdout)?;
    }
    cleanup_terminal(&stdout)?;
    Ok(())
}
