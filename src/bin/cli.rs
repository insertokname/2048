use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal,
};
use game_2048::game;
use std::io::{stdout, Write};

mod format;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    crossterm::execute!(stdout, Print("Press: 'q' to exit; 'r' to undo\n\r"))?;

    let mut undo_game: Option<game::Game> = None;
    let mut game = game::Game::new(2).map_err(|err| {
        write!(stdout, "{err}\n\r").unwrap();
        stdout.flush().unwrap();
        err
    })?;

    format::format_print_game(&mut stdout, &game)?;

    loop {
        match event::read()? {
            Event::Key(pressed_key) => {
                let mut force_redraw = false;
                let mut direction: Option<game::Direction> = None;

                if pressed_key == KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)
                    || pressed_key == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)
                    || pressed_key == KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE)
                {
                    break;
                }

                if pressed_key == KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE) {
                    if let Some(undo_game) = undo_game.take() {
                        game = undo_game;
                        force_redraw = true;
                    }
                }

                if !game.board_ref().is_dead() {
                    if pressed_key == KeyEvent::new(KeyCode::Left, KeyModifiers::NONE) {
                        direction = Some(game::Direction::Left);
                    } else if pressed_key == KeyEvent::new(KeyCode::Right, KeyModifiers::NONE) {
                        direction = Some(game::Direction::Right);
                    } else if pressed_key == KeyEvent::new(KeyCode::Up, KeyModifiers::NONE) {
                        direction = Some(game::Direction::Up);
                    } else if pressed_key == KeyEvent::new(KeyCode::Down, KeyModifiers::NONE) {
                        direction = Some(game::Direction::Down);
                    }

                    if let Some(direction) = direction {
                        force_redraw = true;

                        let clone = game.clone();
                        if let Ok(_) = game.play(direction) {
                            undo_game = Some(clone);
                        }
                    }
                }

                if force_redraw {
                    crossterm::execute!(stdout, cursor::MoveUp(5), cursor::MoveToColumn(0))?;
                    format::format_print_game(&mut stdout, &game)?;
                    stdout.flush()?;
                }
            }
            _ => (),
        }
    }

    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}
