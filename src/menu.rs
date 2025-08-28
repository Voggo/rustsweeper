use crate::game::{GameConfig, MenuItem, MenuItemType};
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
                todo!("Handle right key for custom menu");
            }
            event::KeyCode::Left => {
                todo!("Handle left key for custom menu");
            }
            _ => {}
        }
    }
}

const MAIN_MENU_ITEMS_LIST: [MenuItem; 5] = [
    MenuItem {
        item_type: MenuItemType::Beginnner,
        name: "Beginner",
        config: Some(GameConfig {
            width: 9,
            height: 9,
            mines: 10,
        }),
    },
    MenuItem {
        item_type: MenuItemType::Intermediate,
        name: "Intermediate",
        config: Some(GameConfig {
            width: 16,
            height: 16,
            mines: 40,
        }),
    },
    MenuItem {
        item_type: MenuItemType::Expert,
        name: "Expert",
        config: Some(GameConfig {
            width: 30,
            height: 16,
            mines: 99,
        }),
    },
    MenuItem {
        item_type: MenuItemType::Custom,
        name: "Custom",
        config: None,
    },
    MenuItem {
        item_type: MenuItemType::Exit,
        name: "Exit",
        config: None,
    },
];

const CUSTOM_MENU_ITEMS_LIST: [MenuItem; 3] = [
    MenuItem {
        item_type: MenuItemType::Custom,
        name: "Width",
        config: None,
    },
    MenuItem {
        item_type: MenuItemType::Custom,
        name: "Height",
        config: None,
    },
    MenuItem {
        item_type: MenuItemType::Custom,
        name: "Mines",
        config: None,
    },
];
