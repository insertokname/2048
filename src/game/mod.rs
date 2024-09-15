use crate::board;
use error::GameError;
use rand::Rng;

mod error;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Game {
    board: board::Board,
}

impl Game {
    pub fn new(default_spaces: i32) -> Result<Game, Box<dyn std::error::Error>> {
        let mut game = Game {
            board: board::Board::new(),
        };
        for _ in 0..default_spaces {
            game.generate()?;
        }
        Ok(game)
    }

    fn generate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut avaliable_pos: Vec<(usize, usize)> = Vec::new();
        avaliable_pos.reserve_exact(board::BOARD_SIZE * board::BOARD_SIZE);
        for i in 0..board::BOARD_SIZE {
            for j in 0..board::BOARD_SIZE {
                if self.board.board[i][j] == 0 {
                    avaliable_pos.push((i, j));
                }
            }
        }

        if avaliable_pos.is_empty() {
            return Err(Box::new(GameError::GenerateError(
                GameError::GameDeadError.into(),
            )));
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..avaliable_pos.len());
        let final_pos = avaliable_pos[index];
        self.board.board[final_pos.0][final_pos.1] = if rng.gen_ratio(1, 10) { 4 } else { 2 };
        Ok(())
    }

    pub fn play(&mut self, direction: Direction) -> Result<(), Box<dyn std::error::Error>> {
        if self.board.is_dead() {
            return Err(Box::new(GameError::PlayError(
                GameError::GameDeadError.into(),
            )));
        }
        match direction {
            Direction::Up => self.board.shift_up(),
            Direction::Down => self.board.shift_down(),
            Direction::Left => self.board.shift_left(),
            Direction::Right => self.board.shift_right(),
        }?;

        self.generate()?;

        Ok(())
    }

    pub fn board_ref(&self) -> &board::Board {
        return &self.board;
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}
