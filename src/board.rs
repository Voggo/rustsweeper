use crate::game::*;
use crossterm::event;
use rand::prelude::*;

pub struct Board {
    grid: Vec<CellBox>,
    pub width: usize,
    pub height: usize,
    mines_placed: bool,
    mines_to_place: usize,
}

impl Board {
    pub fn new() -> Board {
        let grid = vec![
            CellBox {
                kind: CellKind::Number(0),
                state: CellState::Hidden,
            };
            DEFAULT_CONFIG.width * DEFAULT_CONFIG.height
        ];
        Board {
            grid,
            width: DEFAULT_CONFIG.width,
            height: DEFAULT_CONFIG.height,
            mines_to_place: DEFAULT_CONFIG.mines,
            mines_placed: false,
        }
    }

    pub fn new_with_config(config: GameConfig) -> Board {
        let grid = vec![
            CellBox {
                kind: CellKind::Number(0),
                state: CellState::Hidden,
            };
            config.width * config.height
        ];
        Board {
            grid,
            width: config.width,
            height: config.height,
            mines_to_place: config.mines,
            mines_placed: false,
        }
    }

    pub fn initialize_board(&mut self, initial_click_x: isize, initial_click_y: isize) {
        let mut rng = rand::rng();
        let mut set_index = (0..(self.width * self.height)).collect::<Vec<usize>>();
        set_index.shuffle(&mut rng);
        for i in 0..self.mines_to_place {
            let mut idx = set_index[i];
            if (idx % self.width) as isize == initial_click_x
                && (idx as isize - initial_click_x) / self.width as isize == initial_click_y
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

    pub fn get_cell(&self, x: isize, y: isize) -> Option<&CellBox> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y * self.width as isize + x;
            Some(&self.grid[index as usize])
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut CellBox> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y * self.width as isize + x;
            Some(&mut self.grid[index as usize])
        } else {
            None
        }
    }
    pub fn get_board_start_pos(&self) -> (u16, u16) {
        let (cols, rows) = crossterm::terminal::size().expect("Failed to get terminal size");
        let board_start_x = (cols as i16 - (self.width * 2 + 2) as i16) / 2;
        let board_start_y = (rows as i16 - self.height as i16) / 2;
        (board_start_x as u16, board_start_y as u16)
    }
    pub fn check_win_condition(&self) -> Option<GameState> {
        for cell in self.grid.iter() {
            if cell.kind != CellKind::Mine && cell.state != CellState::Revealed {
                return None; // Found a non-mine cell that is not revealed
            }
        }
        Some(GameState::Won) // All non-mine cells are revealed
    }
    pub fn reveal_adjacent_empty(&mut self, x: isize, y: isize) -> Option<GameState> {
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
    pub fn reveal_non_flagged(&mut self, x: isize, y: isize) -> Option<GameState> {
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
    pub fn handle_mouse_left(&mut self, event: event::MouseEvent) -> Option<GameState> {
        let (board_start_x, board_start_y) = self.get_board_start_pos();
        // Offset mouse coordinates by border (1 for top, 2 for left: border + space)
        let cell_x = ((event.column as isize - board_start_x as isize - 2) / 2).max(0);
        let cell_y = (event.row as isize - board_start_y as isize - 1).max(0);
        if self.mines_placed == false {
            self.initialize_board(cell_x, cell_y);
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
        }
        None
    }
    pub fn handle_mouse_right(&mut self, event: event::MouseEvent) {
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
        }
    }
}
