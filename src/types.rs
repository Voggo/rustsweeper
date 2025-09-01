use ::crossterm::style::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Ongoing,
    Won,
    Lost,
    Exit,
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
pub enum MenuItemType {
    Beginnner,
    Intermediate,
    Expert,
    Custom,
    Exit,
    Width,
    Height,
    Mines,
    Confirm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItem {
    Main {
        item_type: MenuItemType,
        name: &'static str,
        config: Option<GameConfig>,
    },
    Custom {
        item_type: MenuItemType,
        name: &'static str,
        value: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellBox {
    pub kind: CellKind,
    pub state: CellState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub const MIN_WIDTH: usize = 5;
pub const MAX_WIDTH: usize = 50;
pub const MIN_HEIGHT: usize = 5;
pub const MAX_HEIGHT: usize = 50;
pub const MIN_MINES: usize = 1;

pub const DEFAULT_CONFIG: GameConfig = GameConfig {
    // fallback config
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
