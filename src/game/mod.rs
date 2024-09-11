use crate::board;

use rand::Rng;

pub struct Game {
    pub board: board::Board,
    score: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: board::Board::new(),
            score: 0,
        }
        .generate()
        .generate()
    }

    pub fn generate(mut self) -> Self {
        let mut avaliable_pos: Vec<(usize, usize)> = Vec::new();
        avaliable_pos.reserve_exact(board::BOARD_SIZE * board::BOARD_SIZE);
        for i in 0..board::BOARD_SIZE {
            for j in 0..board::BOARD_SIZE {
                if self.board.board[i][j] == 0 {
                    avaliable_pos.push((i, j));
                }
            }
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..avaliable_pos.len());
        let final_pos = avaliable_pos[index];
        self.board.board[final_pos.0][final_pos.1] = if rng.gen_ratio(1, 10) { 4 } else { 2 };
        self
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score: {}\n{}", self.score, self.board)
    }
}
