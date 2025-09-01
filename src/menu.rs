use crate::game_logic::Board;
use crate::types::{GameConfig, GameState, MenuItem, MenuItemType};
use crossterm::event;

#[derive(Debug, Clone)]
pub struct Menu {
    pub items: Vec<MenuItem>,
    pub hovered_index: usize,
    pub selected: Option<MenuItem>,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Menu {
        Menu {
            items,
            hovered_index: 0,
            selected: None,
        }
    }
    pub fn new_main_menu() -> Menu {
        Menu::new(MAIN_MENU_ITEMS_LIST.to_vec())
    }
    pub fn new_custom_menu() -> Menu {
        Menu::new(CUSTOM_MENU_ITEMS_LIST.to_vec())
    }
    pub fn get_hovered_item(&self) -> &MenuItem {
        &self.items[self.hovered_index]
    }
    pub fn select(&mut self) {
        let selected_item = self.get_hovered_item().clone();
        self.selected = Some(selected_item);
    }
    pub fn next(&mut self) {
        self.hovered_index = (self.hovered_index + 1) % self.items.len();
    }
    pub fn previous(&mut self) {
        if self.hovered_index == 0 {
            self.hovered_index = self.items.len() - 1;
        } else {
            self.hovered_index -= 1;
        }
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn get_custom_config(&self) -> Option<GameConfig> {
        let mut config = GameConfig {
            width: 0,
            height: 0,
            mines: 0,
        };
        for item in &self.items {
            if let MenuItem::Custom {
                item_type, value, ..
            } = item
            {
                match item_type {
                    MenuItemType::Width => config.width = *value,
                    MenuItemType::Height => config.height = *value,
                    MenuItemType::Mines => config.mines = *value,
                    _ => {}
                }
            }
        }
        // Check if all values were found
        if config.width > 0 && config.height > 0 {
            Some(config)
        } else {
            None
        }
    }
}

pub fn handle_menu_event(event: &event::Event, menu: &mut Menu) {
    if let event::Event::Key(key_event) = event {
        match key_event.code {
            event::KeyCode::Down => {
                menu.next();
            }
            event::KeyCode::Up => {
                menu.previous();
            }
            event::KeyCode::Enter => {
                menu.select();
            }
            event::KeyCode::Right => {
                // Increase value for custom menu item
                if let MenuItem::Custom {
                    item_type,
                    name,
                    value,
                } = menu.items[menu.hovered_index]
                {
                    let new_value = value.saturating_add(1); // or set a max limit
                    menu.items[menu.hovered_index] = MenuItem::Custom {
                        item_type,
                        name,
                        value: new_value,
                    };
                }
            }
            event::KeyCode::Left => {
                // Decrease value for custom menu item
                if let MenuItem::Custom {
                    item_type,
                    name,
                    value,
                } = menu.items[menu.hovered_index]
                {
                    let new_value = value.saturating_sub(1).max(1); // or set a min limit
                    menu.items[menu.hovered_index] = MenuItem::Custom {
                        item_type,
                        name,
                        value: new_value,
                    };
                }
            }
            _ => {}
        }
    }
}

pub fn process_menu_selection(
    current_menu: &mut Menu,
    board: &mut Board,
    game_state: &mut GameState,
) {
    if let Some(item) = current_menu.selected {
        match item {
            MenuItem::Main {
                item_type, config, ..
            } => match item_type {
                MenuItemType::Beginnner | MenuItemType::Intermediate | MenuItemType::Expert => {
                    *board = Board::new_with_config(config.unwrap());
                    *game_state = GameState::Ongoing;
                }
                MenuItemType::Custom => {
                    *current_menu = Menu::new_custom_menu();
                }
                MenuItemType::Exit => {
                    *game_state = GameState::Exit; // Or some exit state
                }
                MenuItemType::Confirm => {
                    if let Some(config) = current_menu.get_custom_config() {
                        if config.mines >= config.width * config.height {
                            // Handle error: maybe reset menu to defaults
                            // Or better yet, prevent this state in the key handlers!
                            return;
                        }
                        *board = Board::new_with_config(Board::clamp_config(
                            config.width,
                            config.height,
                            config.mines,
                        ));
                        *game_state = GameState::Ongoing;
                    }
                }
                _ => {}
            },
            MenuItem::Custom { .. } => {
                // Do nothing for now
            }
        }
    }
}

const MAIN_MENU_ITEMS_LIST: [MenuItem; 5] = [
    MenuItem::Main {
        item_type: MenuItemType::Beginnner,
        name: "Beginner",
        config: Some(GameConfig {
            width: 9,
            height: 9,
            mines: 10,
        }),
    },
    MenuItem::Main {
        item_type: MenuItemType::Intermediate,
        name: "Intermediate",
        config: Some(GameConfig {
            width: 16,
            height: 16,
            mines: 40,
        }),
    },
    MenuItem::Main {
        item_type: MenuItemType::Expert,
        name: "Expert",
        config: Some(GameConfig {
            width: 30,
            height: 16,
            mines: 99,
        }),
    },
    MenuItem::Main {
        item_type: MenuItemType::Custom,
        name: "Custom",
        config: None,
    },
    MenuItem::Main {
        item_type: MenuItemType::Exit,
        name: "Exit",
        config: None,
    },
];

const CUSTOM_MENU_ITEMS_LIST: [MenuItem; 4] = [
    MenuItem::Custom {
        item_type: MenuItemType::Width,
        name: "Width",
        value: 30,
    },
    MenuItem::Custom {
        item_type: MenuItemType::Height,
        name: "Height",
        value: 30,
    },
    MenuItem::Custom {
        item_type: MenuItemType::Mines,
        name: "Mines",
        value: 150,
    },
    MenuItem::Main {
        item_type: MenuItemType::Confirm,
        name: "Confirm",
        config: None,
    },
];
