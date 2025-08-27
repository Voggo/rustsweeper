use rust_sweeper::{
    board::Board,
    game::{GameState, MENU_ITEMS_LIST, MenuItemType},
    menu,
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

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = stdout();
    setup_terminal(&stdout)?;

    let mut board = Board::new();
    let mut game_state = GameState::InMenu;
    let mut current_menu_select = MENU_ITEMS_LIST[0];

    tui::set_styles(&stdout)?;
    'game_loop: loop {
        match game_state {
            GameState::InMenu => {
                tui::render_game_menu(&mut stdout, current_menu_select)?;
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
            GameState::InMenu => {
                current_menu_select = menu::choose_menu(&event, current_menu_select);
                if current_menu_select.selected {
                    match current_menu_select.item_type {
                        MenuItemType::Beginnner
                        | MenuItemType::Intermediate
                        | MenuItemType::Expert => {
                            board = Board::new_with_config(current_menu_select.config);
                            game_state = GameState::Ongoing;
                            render_game_board(&board, &mut stdout)?;
                            continue;
                        }
                        MenuItemType::Exit => {
                            break 'game_loop;
                        }
                        _ => {}
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
                if should_restart(&event) {
                    current_menu_select = MENU_ITEMS_LIST[0];
                    game_state = GameState::InMenu;
                }
            }
        }
    }

    cleanup_terminal(&stdout)?;
    Ok(())
}
