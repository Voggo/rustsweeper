use crate::game::{MENU_ITEMS_LIST, MenuItem};
use crossterm::event;

pub fn choose_menu(event: &event::Event, current_item: MenuItem) -> MenuItem {
    if let event::Event::Key(key_event) = event {
        match key_event.code {
            event::KeyCode::Up => {
                let idx = MENU_ITEMS_LIST
                    .iter()
                    .position(|item| item == &current_item)
                    .unwrap_or(0);
                let new_idx = if idx == 0 {
                    MENU_ITEMS_LIST.len() - 1
                } else {
                    idx - 1
                };
                MENU_ITEMS_LIST[new_idx]
            }
            event::KeyCode::Down => {
                let idx = MENU_ITEMS_LIST
                    .iter()
                    .position(|item| item == &current_item)
                    .unwrap_or(0);
                let new_idx = if idx + 1 >= MENU_ITEMS_LIST.len() {
                    0
                } else {
                    idx + 1
                };
                MENU_ITEMS_LIST[new_idx]
            }
            event::KeyCode::Enter => MenuItem {
                item_type: current_item.item_type,
                name: current_item.name,
                config: current_item.config,
                selected: true,
            },
            _ => current_item,
        }
    } else {
        current_item
    }
}
