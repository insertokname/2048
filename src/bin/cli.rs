use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, Stylize},
    terminal,
};
use game_2048::{board, game};
use std::io::{stdout, Write};

fn pretty_print_game(game: &game::Game) {
    print!("Score: {} ", game.board_ref().score);
    if game.board_ref().is_dead() {
        print!("{}", "You died!".with(Color::White).on(Color::Red))
    } else if game.is_game_won() {
        print!("{}", "You won! ".with(Color::White).on(Color::Yellow))
    } else {
        print!("{}", "         ")
    }
    print!("\n\r");
    for i in 0..board::BOARD_SIZE {
        for j in 0..board::BOARD_SIZE {
            let value = game.board_ref().board[i][j];
            let output = match value {
                0 => format!("  {}   ", value.to_string())
                    .with(Color::Black)
                    .on(Color::White),

                2 => format!("  {}   ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Yellow),

                4 => format!("  {}   ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Yellow),

                8 => format!("  {}   ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Cyan),

                16 => format!("  {}  ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Cyan),

                32 => format!("  {}  ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Blue),

                64 => format!("  {}  ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Blue),

                128 => format!(" {}  ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Magenta),

                256 => format!(" {}  ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Magenta),

                512 => format!(" {}  ", value.to_string())
                    .with(Color::Black)
                    .on(Color::Red),

                1024 => format!(" {} ", value.to_string())
                    .with(Color::Black)
                    .on(Color::DarkRed),

                _ => format!(" {} ", value.to_string()).with(Color::Black),
            };

            print!("{}", output);
        }
        print!("\n\r");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    if let Err(err) = crossterm::execute!(stdout, Print("Press: 'q' to exit; 'r' to undo\n\r"),) {
        print!("{err}");
        return Err(Box::new(err));
    }

    let mut old_game: Option<game::Game> = None;
    let mut game = game::Game::new(2).map_err(|err| {
        write!(stdout, "{err}\n\r").unwrap();
        stdout.flush().unwrap();
        err
    })?;

    // let mut game = game::Game::new_test();

    pretty_print_game(&game);

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
                    if let Some(old_game) = old_game.take() {
                        game = old_game;
                        force_redraw = true;
                    }
                }

                if game.board_ref().is_dead() {
                    force_redraw = true;
                } else {
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
                            old_game = Some(clone);
                        }
                    }
                }

                if force_redraw {
                    crossterm::execute!(stdout, cursor::MoveUp(5), cursor::MoveToColumn(0))?;
                    pretty_print_game(&game);
                }
            }
            _ => (),
        }
    }

    stdout.flush()?;
    terminal::disable_raw_mode()?;
    Ok(())
}
