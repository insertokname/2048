use crossterm::style::{Color, Stylize};
use game_2048::{board, game};

pub fn format_print_game(stdout: &mut impl std::io::Write ,game: &game::Game)-> std::io::Result<()> {
    write!(stdout, "Score: {} ", game.board_ref().score)?;
    if game.board_ref().is_dead() {
        write!(stdout, "{}", "You died!".with(Color::White).on(Color::Red))?
    } else if game.board_ref().is_board_won() {
        write!(stdout, "{}", "You won! ".with(Color::White).on(Color::Yellow))?
    } else {
        write!(stdout, "{}", "         ")?
    }
    write!(stdout, "\n\r")?;
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

            write!(stdout, "{}", output)?;
        }
        write!(stdout, "\n\r")?;
    }
    Ok(())
}
