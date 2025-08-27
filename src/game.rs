// should maybe just be renamed to types.rs or something
use ::crossterm::style::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    InMenu,
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
pub enum MenuItemType {
    Beginnner,
    Intermediate,
    Expert,
    Custom,
    Exit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MenuItem {
    pub item_type: MenuItemType,
    pub name: &'static str,
    pub config: GameConfig,
    pub selected: bool,
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

pub const MENU_ITEMS_LIST: [MenuItem; 5] = [
    MenuItem {
        item_type: MenuItemType::Beginnner,
        name: "Beginner",
        config: GameConfig {
            width: 9,
            height: 9,
            mines: 10,
        },
        selected: false,
    },
    MenuItem {
        item_type: MenuItemType::Intermediate,
        name: "Intermediate",
        config: GameConfig {
            width: 16,
            height: 16,
            mines: 40,
        },
        selected: false,
    },
    MenuItem {
        item_type: MenuItemType::Expert,
        name: "Expert",
        config: GameConfig {
            width: 30,
            height: 16,
            mines: 99,
        },
        selected: false,
    },
    MenuItem {
        item_type: MenuItemType::Custom,
        name: "Custom",
        config: GameConfig {
            width: 20,
            height: 20,
            mines: 25,
        },
        selected: false,
    },
    MenuItem {
        item_type: MenuItemType::Exit,
        name: "Exit",
        config: GameConfig {
            width: 0,
            height: 0,
            mines: 0,
        },
        selected: false,
    },
];

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
