use ::crossterm::style::Color;

// Make everything public (`pub`) so other modules can use it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Won,
    Lost,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Mine,
    Number(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellBox {
    pub kind: CellKind,
    pub state: CellState,
}

pub struct GameConfig {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

pub struct ColorConfig {
    pub background: Color,
    pub hidden_cell: Color,
    pub flagged_cell: Color,
    pub mine: Color,
    pub empty_cell: Color,
    pub number: [Color; 8],
    pub border: Color,
}

pub const GAME_CONFIG: GameConfig = GameConfig {
    width: 20,
    height: 20,
    mines: 25,
};

pub const COLOR_CONFIG: ColorConfig = ColorConfig {
    background: Color::White,
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
