use crate::timer::Timer;
use crate::types::*;
use crossterm::event;
use rand::prelude::*;

pub struct Board {
    grid: Vec<CellBox>,
    pub width: usize,
    pub height: usize,
    pub timer: Timer,
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
            timer: Timer::new(),
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
            timer: Timer::new(),
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
        self.timer.start();
    }

    pub fn reset(&mut self) {
        self.grid.iter_mut().for_each(|cell| {
            cell.kind = CellKind::Number(0);
            cell.state = CellState::Hidden;
        });
        self.mines_placed = false;
        self.timer.reset();
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
    fn check_win_condition(&mut self) -> Option<GameState> {
        for cell in self.grid.iter() {
            if cell.kind != CellKind::Mine && cell.state != CellState::Revealed {
                return None; // Found a non-mine cell that is not revealed
            }
        }
        self.timer.stop();
        Some(GameState::Won) // All non-mine cells are revealed
    }
    // some bug here where it does not reveal empty in some scenarios
    pub fn reveal_adjacent_empty(&mut self, x: isize, y: isize) -> Option<GameState> {
        let mut to_reveal = vec![(x, y)];
        while let Some((cx, cy)) = to_reveal.pop() {
            if let Some(cell) = self.get_cell_mut(cx, cy) {
                match cell.state {
                    CellState::Revealed | CellState::Flagged => continue, // Skip if already revealed or flagged
                    CellState::Hidden => {
                        cell.state = CellState::Revealed;
                        if cell.kind == CellKind::Mine {
                            self.timer.stop();
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
                            self.timer.stop();
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
    pub fn reveal_all_mines(&mut self) {
        for cell in self.grid.iter_mut() {
            if cell.kind == CellKind::Mine {
                cell.state = CellState::Revealed;
            }
        }
    }
    fn cell_coords_from_mouse(&self, event: event::MouseEvent) -> Option<(isize, isize)> {
        let (board_start_x, board_start_y) = self.get_board_start_pos();

        let cell_x = (event.column as isize - board_start_x as isize - 2) / 2;
        let cell_y = event.row as isize - board_start_y as isize - 1;

        if cell_x >= 0
            && cell_x < self.width as isize
            && cell_y >= 0
            && cell_y < self.height as isize
        {
            Some((cell_x, cell_y))
        } else {
            None // Click was outside the board
        }
    }
    pub fn handle_mouse_left(&mut self, event: event::MouseEvent) -> Option<GameState> {
        if let Some((cell_x, cell_y)) = self.cell_coords_from_mouse(event) {
            if self.mines_placed == false {
                self.initialize_board(cell_x, cell_y);
                self.mines_placed = true;
            }
            if let Some(cell) = self.get_cell_mut(cell_x, cell_y) {
                match cell.state {
                    CellState::Hidden => {
                        // Only reveal if the cell is hidden
                        if let Some(game_state_change) = self.reveal_adjacent_empty(cell_x, cell_y)
                        {
                            return Some(game_state_change);
                        };
                    }
                    CellState::Revealed => {
                        // If already revealed, reveal adjacent non-flagged cells if it's a number
                        if let CellKind::Number(n) = cell.kind {
                            if n > 0 {
                                if let Some(game_state_change) =
                                    self.reveal_non_flagged(cell_x, cell_y)
                                {
                                    return Some(game_state_change);
                                };
                            }
                        }
                    }
                    _ => (), // Do nothing if it's already revealed or flagged
                };
            }
        }
        None
    }
    pub fn handle_mouse_right(&mut self, event: event::MouseEvent) {
        if let Some((cell_x, cell_y)) = self.cell_coords_from_mouse(event) {
            if let Some(cell) = self.get_cell_mut(cell_x, cell_y) {
                cell.state = match cell.state {
                    CellState::Hidden => CellState::Flagged,
                    CellState::Flagged => CellState::Hidden,
                    _ => cell.state, // Do nothing if it's already revealed
                };
            }
        }
    }
    pub fn clamp_config(width: usize, height: usize, mines: usize) -> GameConfig {
        let clamped_width = width.clamp(MIN_WIDTH, MAX_WIDTH);
        let clamped_height = height.clamp(MIN_HEIGHT, MAX_HEIGHT);
        let max_mines = clamped_width * clamped_height - 1; // Ensure at least one cell is free
        let clamped_mines = mines.clamp(MIN_MINES, max_mines);
        GameConfig {
            width: clamped_width,
            height: clamped_height,
            mines: clamped_mines,
        }
    }
    pub fn get_remaining_mines(&self) -> isize {
        self.mines_to_place as isize - self.get_flags_count() as isize
    }
    // could maybe cache this in the struct instead of calculating it every time
    fn get_flags_count(&self) -> usize {
        self.grid
            .iter()
            .filter(|cell| cell.state == CellState::Flagged)
            .count()
    }
}
