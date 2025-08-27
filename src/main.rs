use rust_sweeper::{
    board::Board,
    game::GameState,
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
    tui::set_styles(&stdout)?;

    let mut board = Board::new();
    let mut game_state = GameState::Ongoing;

    'game_loop: loop {
        match game_state {
            GameState::Ongoing => {
                render_game_board(&board, &mut stdout)?;
                let event = event::read()?;

                if should_exit(&event) {
                    break 'game_loop;
                }

                if let Event::Mouse(mouse_event) = event {
                    // cursor_position = (mouse_event.row, mouse_event.column);
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
                render_game_board(&board, &mut stdout)?;
                tui::overlay_ascii_art(&mut stdout, &board, game_state == GameState::Won)?;

                let event = event::read()?;
                if should_exit(&event) {
                    break 'game_loop;
                }
                if should_restart(&event) {
                    board = Board::new();
                    game_state = GameState::Ongoing;
                }
            }
        }
    }

    cleanup_terminal(&stdout)?;
    Ok(())
}
