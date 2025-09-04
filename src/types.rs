use ::crossterm::style::Color;

///! Types and configuration for termsweeper.
///!
///! This module defines the core types used for game state, board cells, menu items,
///! and color configuration for the terminal UI.

/// Represents the overall state of the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    /// The main menu is active.
    Menu,
    /// The game is currently being played.
    Ongoing,
    /// The player has won the game.
    Won,
    /// The player has lost the game.
    Lost,
    /// The game is exiting.
    Exit,
}

/// Represents the state of a cell on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

/// Represents the kind of a cell (mine or number).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Mine,
    Number(u8),
}

/// Represents the type of a menu item.
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

/// Represents a menu item in the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItem {
    /// Main menu items (Beginner, Itermediate, Expert, Custom, Exit)
    Main {
        item_type: MenuItemType,
        name: &'static str,
        config: Option<GameConfig>,
    },
    /// Custom menu item (width, height, mines).
    Custom {
        item_type: MenuItemType,
        name: &'static str,
        value: usize,
    },
}

/// Represents a cell on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellBox {
    /// The kind of cell (mine or number).
    pub kind: CellKind,
    /// The state of the cell (hidden, revealed, flagged).
    pub state: CellState,
}

/// Configuration for a Minesweeper game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameConfig {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

/// Color configuration for the terminal UI.
pub struct ColorConfig {
    pub background: Color,
    pub hidden_cell: Color,
    pub flagged_cell: Color,
    pub mine: Color,
    pub empty_cell: Color,
    pub number: [Color; 8],
    pub border: Color,
    pub counter: Color,
}

/// Minimum allowed board width.
pub const MIN_WIDTH: usize = 5;
/// Maximum allowed board width.
pub const MAX_WIDTH: usize = 50;
/// Minimum allowed board height.
pub const MIN_HEIGHT: usize = 5;
/// Maximum allowed board height.
pub const MAX_HEIGHT: usize = 50;
/// Minimum allowed number of mines.
pub const MIN_MINES: usize = 1;

/// Default game configuration (used as fallback).
pub const DEFAULT_CONFIG: GameConfig = GameConfig {
    width: 20,
    height: 20,
    mines: 25,
};

/// Default color configuration for the UI.
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
    counter: Color::Blue,
};
