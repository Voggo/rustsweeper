use rand::prelude::*;

use std::io::Write;
use std::io::{Stdout, stdout};

use crossterm::terminal::Clear;
use crossterm::{
    cursor::{MoveTo, RestorePosition, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
    },
    style::*,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    {execute, queue},
};

struct ColorConfig {
    hidden_cell: Color,
    flagged_cell: Color,
    mine: Color,
    empty_cell: Color,
    number: [Color; 8],
    border: Color,
}

const COLOR_CONFIG: ColorConfig = ColorConfig {
    hidden_cell: Color::DarkGrey,
    flagged_cell: Color::DarkRed,
    mine: Color::Black,
    empty_cell: Color::Black,
    number: [
        Color::Blue,       // 1
        Color::Green,      // 2
        Color::Red,        // 3
        Color::Magenta,    // 4
        Color::DarkYellow, // 5
        Color::Cyan,       // 6
        Color::White,      // 7
        Color::Grey,       // 8
    ],
    border: Color::Black,
};

struct GameConfig {
    width: usize,
    height: usize,
    mines: usize,
}

const GAME_CONFIG: GameConfig = GameConfig {
    width: 30,
    height: 30,
    mines: 60,
};

enum GameState {
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
    fn new() -> Board {
        let grid = vec![
            CellBox {
                kind: CellKind::Number(0),
                state: CellState::Hidden,
            };
            GAME_CONFIG.width * GAME_CONFIG.height
        ];
        Board {
            grid,
            width: GAME_CONFIG.width,
            height: GAME_CONFIG.height,
            mines_to_place: GAME_CONFIG.mines,
            mines_placed: false,
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
        let board_start_x = (cols as i16 - (self.width * 2 + 2) as i16) / 2;
        let board_start_y = (rows as i16 - self.height as i16) / 2;
        (board_start_x as u16, board_start_y as u16)
    }
    fn check_win_condition(&self) -> Option<GameState> {
        for cell in self.grid.iter() {
            if cell.kind != CellKind::Mine && cell.state != CellState::Revealed {
                return None; // Found a non-mine cell that is not revealed
            }
        }
        Some(GameState::Won) // All non-mine cells are revealed
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
        return self.check_win_condition();
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
        if ret.is_none() {
            ret = self.check_win_condition();
        }
        return ret;
    }
    fn handle_mouse_left(&mut self, event: event::MouseEvent) -> Option<GameState> {
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        // Offset mouse coordinates by border (1 for top, 2 for left: border + space)
        let cell_x = ((event.column as isize - board_start_x as isize - 2) / 2).max(0);
        let cell_y = (event.row as isize - board_start_y as isize - 1).max(0);
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
        // Offset mouse coordinates by border (1 for top, 2 for left: border + space)
        let cell_x = ((event.column as isize - board_start_x as isize - 2) / 2).max(0);
        let cell_y = (event.row as isize - board_start_y as isize - 1).max(0);
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
        SetBackgroundColor(Color::White),
        SetForegroundColor(Color::Black),
        Clear(terminal::ClearType::All)
    )?;
    return Ok(());
}

fn cleanup_terminal(mut stdout: &Stdout) -> Result<(), std::io::Error> {
    execute!(
        stdout,
        Show,
        LeaveAlternateScreen,
        DisableMouseCapture,
        ResetColor,
        RestorePosition
    )?;
    terminal::disable_raw_mode()?;
    return Ok(());
}

/// Overlay ASCII art above the finished game board for win/lose screens.
/// If there is room, place it above the board; otherwise, center in terminal.
fn overlay_ascii_art(stdout: &mut Stdout, board: &Board, win: bool) -> anyhow::Result<()> {
    let win_art = [
        " __     __          __          ___       ",
        " \\ \\   / /          \\ \\        / (_)      ",
        "  \\ \\_/ /__  _   _   \\ \\  /\\  / / _ _ __  ",
        "   \\   / _ \\| | | |   \\ \\/  \\/ / | | '_ \\ ",
        "    | | (_) | |_| |    \\  /\\  /  | | | | |",
        "    |_|\\___/ \\__,_|     \\/  \\/   |_|_| |_|",
    ];
    let lose_art = [
        "  _____                         ____                 ",
        " / ____|                       / __ \\                ",
        "| |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __ ",
        "| | |_ |/ _` | '_ ` _ \\ / _ \\ | |  | \\ \\ / / _ \\ '__|",
        "| |__| | (_| | | | | | |  __/ | |__| |\\ V /  __/ |   ",
        " \\_____|\\__,_|_| |_| |_|\\___|  \\____/  \\_/ \\___|_|   ",
    ];
    let art = if win { &win_art } else { &lose_art };
    let color = if win { Color::Green } else { Color::Red };

    let (cols, rows) = crossterm::terminal::size().expect("Failed to get terminal size");
    let art_width = art[0].len() as u16;
    let art_height = art.len() as u16;
    let art_x = (cols.saturating_sub(art_width)) / 2;

    let (_, board_start_y) = board.get_board_start_pos();
    let art_y = if board_start_y >= art_height + 1 {
        board_start_y - art_height - 1
    } else {
        (rows.saturating_sub(art_height)) / 2
    };

    for (i, line) in art.iter().enumerate() {
        queue!(
            stdout,
            SetForegroundColor(color),
            MoveTo(art_x, art_y + i as u16),
            Print(line),
        )?;
    }
    stdout.flush()?;
    Ok(())
}

fn render_game_board(board: &Board, stdout: &mut Stdout) -> anyhow::Result<()> {
    let (board_start_x, board_start_y) = board.get_board_start_pos();
    queue!(
        stdout,
        Clear(terminal::ClearType::All),
        MoveTo(board_start_x, board_start_y)
    )?;
    // Draw top border
    queue!(
        stdout,
        SetForegroundColor(COLOR_CONFIG.border),
        MoveTo(board_start_x, board_start_y),
        Print("┌"),
    )?;
    for _ in 0..board.width {
        queue!(stdout, Print("──"))?;
    }
    queue!(stdout, Print("─┐"))?;

    // Draw board rows with left/right borders
    for y in 0..board.height {
        queue!(
            stdout,
            MoveTo(board_start_x, board_start_y + 1 + y as u16),
            Print("│ "),
        )?;
        for x in 0..board.width {
            // Render cells with offset for border
            if let Some(cell) = board.get_cell(x as isize, y as isize) {
                let (symbol, color) = match cell.state {
                    CellState::Hidden => ("■", COLOR_CONFIG.hidden_cell),
                    CellState::Flagged => ("⚑", COLOR_CONFIG.flagged_cell),
                    CellState::Revealed => match cell.kind {
                        CellKind::Mine => ("💣", COLOR_CONFIG.mine),
                        CellKind::Number(0) => ("  ", COLOR_CONFIG.empty_cell),
                        CellKind::Number(n) => (
                            &n.to_string()[..],
                            COLOR_CONFIG.number[(n as usize).saturating_sub(1).min(7)],
                        ),
                    },
                };
                let symbol = if symbol == "💣" {
                    format!("{:>1}", symbol)
                } else {
                    format!("{:^2}", symbol)
                };
                queue!(stdout, SetForegroundColor(color), Print(symbol))?;
            }
        }
        queue!(stdout, SetForegroundColor(COLOR_CONFIG.border), Print("│"))?;
    }

    // Draw bottom border
    queue!(
        stdout,
        SetForegroundColor(COLOR_CONFIG.border),
        MoveTo(board_start_x, board_start_y + 1 + board.height as u16),
        Print("└"),
    )?;
    for _ in 0..board.width {
        queue!(stdout, Print("──"))?;
    }
    queue!(stdout, Print("─┘"))?;

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
    let mut board = Board::new();
    render_game_board(&board, &mut stdout)?;
    let mut game_state = GameState::Ongoing;
    'game_loop: loop {
        let event = event::read()?;
        if should_exit(&event)? == true {
            break 'game_loop;
        }

        if let Event::Mouse(event) = event {
            // Handle mouse events here if needed
            // println!("Mouse event: {:?}", event);
            match event.kind {
                event::MouseEventKind::Moved => {
                    // Handle mouse movement if needed
                    queue!(stdout, MoveTo(event.column, event.row))?;
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

        match game_state {
            GameState::Ongoing => (),
            GameState::Won => {
                render_game_board(&board, &mut stdout)?;
                overlay_ascii_art(&mut stdout, &board, true)?;
                std::thread::sleep(std::time::Duration::from_secs(3));
                cleanup_terminal(&stdout)?;
                println!("You won! Congratulations!");
                return Ok(());
            }
            GameState::Lost => {
                // Reveal all mines
                for cell in board.grid.iter_mut() {
                    if cell.kind == CellKind::Mine {
                        cell.state = CellState::Revealed;
                    }
                }
                render_game_board(&board, &mut stdout)?;
                overlay_ascii_art(&mut stdout, &board, false)?;
                std::thread::sleep(std::time::Duration::from_secs(3));
                cleanup_terminal(&stdout)?;
                println!("You hit a mine! Game Over!");
                return Ok(());
            }
        }

        render_game_board(&board, &mut stdout)?;
    }
    cleanup_terminal(&stdout)?;
    Ok(())
}
