use rust_sweeper::{
    board::Board,
    game::{GameState, MenuItem, MenuItemType},
    menu::{self, Menu},
    tui::{self, cleanup_terminal, render_game_board, setup_terminal},
};

use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseButton};
use std::io::stdout;

fn should_exit(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key_event) if key_event.code == KeyCode::Esc
            || (key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL)
    )
}

fn should_restart(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key_event) if key_event.code == KeyCode::Char('r')
    )
}

fn should_menu(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key_event) if key_event.code == KeyCode::Char('m')
    )
}

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = stdout();
    setup_terminal(&stdout)?;

    let mut board = Board::new();
    let mut game_state = GameState::Menu;
    let main_menu = Menu::new_main_menu();
    let custom_menu = Menu::new_custom_menu();
    let mut current_menu = Box::new(main_menu);
    tui::set_styles(&stdout)?;
    'game_loop: loop {
        match game_state {
            GameState::Menu => {
                tui::render_game_menu(&mut stdout, &current_menu.clone())?; // todo: avoid clone
            }
            GameState::Ongoing => {
                render_game_board(&board, &mut stdout)?;
            }
            GameState::Won | GameState::Lost => {
                render_game_board(&board, &mut stdout)?;
                tui::overlay_ascii_art(&mut stdout, &board, game_state == GameState::Won)?;
            }
        }
        let event = event::read()?;
        if should_exit(&event) {
            break 'game_loop;
        }
        match game_state {
            GameState::Menu => {
                menu::handle_menu_event(&event, &mut *current_menu);
                if let Some(item) = current_menu.selected {
                    match item {
                        MenuItem::Main {
                            item_type, config, ..
                        } => match item_type {
                            MenuItemType::Beginnner
                            | MenuItemType::Intermediate
                            | MenuItemType::Expert => {
                                // safe to unwrap based on a const assignment in menu.rs
                                board = Board::new_with_config(config.unwrap());
                                game_state = GameState::Ongoing;
                                continue;
                            }
                            MenuItemType::Custom => {
                                current_menu = Box::new(custom_menu.clone());
                                continue;
                            }
                            MenuItemType::Confirm => {
                                // safe to unwrap based on menu logic
                                let width = if let MenuItem::Custom {
                                    item_type: MenuItemType::Width,
                                    value,
                                    ..
                                } = current_menu.items[0]
                                {
                                    value
                                } else {
                                    9
                                };
                                let height = if let MenuItem::Custom {
                                    item_type: MenuItemType::Height,
                                    value,
                                    ..
                                } = current_menu.items[1]
                                {
                                    value
                                } else {
                                    9
                                };
                                let mines = if let MenuItem::Custom {
                                    item_type: MenuItemType::Mines,
                                    value,
                                    ..
                                } = current_menu.items[2]
                                {
                                    value
                                } else {
                                    10
                                };
                                // Validate custom config
                                if mines >= width * height {
                                    // Invalid config, reset to default custom values
                                    current_menu.items[0] = MenuItem::Custom {
                                        item_type: MenuItemType::Width,
                                        name: "Width",
                                        value: 9,
                                    };
                                    current_menu.items[1] = MenuItem::Custom {
                                        item_type: MenuItemType::Height,
                                        name: "Height",
                                        value: 9,
                                    };
                                    current_menu.items[2] = MenuItem::Custom {
                                        item_type: MenuItemType::Mines,
                                        name: "Mines",
                                        value: 10,
                                    };
                                    continue; // stay in custom menu
                                }
                                board = Board::new_with_config(Board::clamp_config(
                                    width, height, mines,
                                ));
                                game_state = GameState::Ongoing;
                                continue;
                            }
                            MenuItemType::Exit => {
                                break 'game_loop;
                            }
                            _ => {
                                continue; // not implemented
                            }
                        },
                        MenuItem::Custom { .. } => {
                            continue; // not implemented
                        }
                    }
                }
            }

            GameState::Ongoing => {
                if let Event::Mouse(mouse_event) = event {
                    match mouse_event.kind {
                        event::MouseEventKind::Down(MouseButton::Left) => {
                            if let Some(new_state) = board.handle_mouse_left(mouse_event) {
                                game_state = new_state;
                            }
                        }
                        event::MouseEventKind::Down(MouseButton::Right) => {
                            board.handle_mouse_right(mouse_event);
                        }
                        _ => {}
                    }
                }
            }
            GameState::Won | GameState::Lost => {
                board.all_mines_revealed();
                if should_restart(&event) {
                    game_state = GameState::Ongoing;
                    board.reset();
                } else if should_menu(&event) {
                    game_state = GameState::Menu;
                    current_menu = Box::new(menu::Menu::new_main_menu());
                }
            }
        }
    }

    cleanup_terminal(&stdout)?;
    Ok(())
}
