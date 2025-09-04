use termsweeper::{
    game_logic::Board,
    menu::{self, Menu},
    tui::{self, cleanup_terminal, render_game_board, setup_terminal},
    types::GameState,
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
    let mut current_menu = Box::new(main_menu);
    tui::set_styles(&stdout)?;
    'game_loop: loop {
        match game_state {
            GameState::Menu => {
                tui::render_game_menu(&mut stdout, &current_menu)?;
            }
            GameState::Ongoing => {
                render_game_board(&board, &mut stdout)?;
            }
            GameState::Won | GameState::Lost => {
                render_game_board(&board, &mut stdout)?;
                tui::overlay_ascii_art(&mut stdout, &board, game_state == GameState::Won)?;
            }
            GameState::Exit => {
                break 'game_loop;
            }
        }
        // Wait for event, but only up to 100ms
        if let Ok(false) = event::poll(std::time::Duration::from_millis(100)) {
            if game_state == GameState::Ongoing {
                render_game_board(&board, &mut stdout)?;
                continue;
            }
        }
        let event = event::read()?;
        if should_exit(&event) {
            break 'game_loop;
        }
        match game_state {
            GameState::Menu => {
                menu::handle_menu_event(&event, &mut *current_menu);
                menu::process_menu_selection(&mut *current_menu, &mut board, &mut game_state);
                if game_state == GameState::Ongoing {
                    continue;
                }
                if game_state == GameState::Lost {
                    break 'game_loop;
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
                board.reveal_all_mines();
                if should_restart(&event) {
                    game_state = GameState::Ongoing;
                    board.reset();
                } else if should_menu(&event) {
                    game_state = GameState::Menu;
                    current_menu = Box::new(menu::Menu::new_main_menu());
                }
            }
            GameState::Exit => {
                break 'game_loop;
            }
        }
    }

    cleanup_terminal(&stdout)?;
    Ok(())
}
