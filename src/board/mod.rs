use error::BoardError;

pub const BOARD_SIZE: usize = 4;

#[derive(Clone)]
pub struct Board {
    pub board: [[u32; BOARD_SIZE]; BOARD_SIZE],
    pub score: u64,
}

mod error;

impl Board {
    pub fn new() -> Board {
        let board = Board {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            score: 0,
        };
        board
    }

    fn shift_horizontal(
        &mut self,
        range: impl Clone + DoubleEndedIterator<Item = usize>,
    ) -> Result<(), BoardError> {
        let mut changed_board: bool = false;
        for i in 0..BOARD_SIZE {
            let mut stack = [0; BOARD_SIZE];
            let mut stack_height = 0;

            for j in range.clone() {
                if stack_height > 0 && self.board[i][j]==0{
                    changed_board=true;
                }
                if self.board[i][j] != 0 {
                    stack[stack_height] = self.board[i][j];
                    stack_height += 1;
                }
            }

            for output_index in range.clone().rev() {
                if stack_height == 0 {
                    self.board[i][output_index] = 0;
                }
                if stack_height == 1 {
                    self.board[i][output_index] = stack[stack_height - 1];
                    stack_height -= 1;
                }
                if stack_height >= 2 {
                    if stack[stack_height - 1] == stack[stack_height - 2] {
                        self.board[i][output_index] = stack[stack_height - 1] << 1;
                        self.score += (stack[stack_height - 1] as u64) << 1;
                        stack_height -= 2;
                        changed_board = true;
                    } else {
                        self.board[i][output_index] = stack[stack_height - 1];
                        stack_height -= 1;
                    }
                }
            }
        }
        if !changed_board{
            return Err(BoardError::UnchangedShiftError);
        }
        Ok(())
    }

    fn shift_vertical(&mut self, range: impl Clone + DoubleEndedIterator<Item = usize>) -> Result<(), BoardError>{
        let mut changed_board: bool = false;
        for i in 0..BOARD_SIZE {
            let mut stack = [0; BOARD_SIZE];
            let mut stack_height = 0;

            for j in range.clone() {
                if stack_height > 0 && self.board[j][i]==0{
                    changed_board=true;
                }
                if self.board[j][i] != 0 {
                    stack[stack_height] = self.board[j][i];
                    stack_height += 1;
                }
            }

            for output_index in range.clone().rev() {
                if stack_height == 0 {
                    self.board[output_index][i] = 0;
                }
                if stack_height == 1 {
                    self.board[output_index][i] = stack[stack_height - 1];
                    stack_height -= 1;
                }
                if stack_height >= 2 {
                    if stack[stack_height - 1] == stack[stack_height - 2] {
                        self.board[output_index][i] = stack[stack_height - 1] << 1;
                        self.score += (stack[stack_height - 1] as u64) << 1;
                        stack_height -= 2;
                        changed_board = true;
                    } else {
                        self.board[output_index][i] = stack[stack_height - 1];
                        stack_height -= 1;
                    }
                }
            }
        }
        if !changed_board{
            return Err(BoardError::UnchangedShiftError);
        }
        Ok(())
    }

    pub fn shift_right(&mut self) -> Result<(), BoardError> {
        self.shift_horizontal(0..BOARD_SIZE)
    }

    pub fn shift_left(&mut self) -> Result<(), BoardError> {
        self.shift_horizontal((0..BOARD_SIZE).rev())
    }

    pub fn shift_up(&mut self) -> Result<(), BoardError> {
        self.shift_vertical((0..BOARD_SIZE).rev())
    }

    pub fn shift_down(&mut self) -> Result<(), BoardError> {
        self.shift_vertical(0..BOARD_SIZE)
    }

    pub fn is_dead(&self) -> bool {
        if self.board.as_flattened().iter().any(|elem| *elem == 0) {
            return false;
        }
        let adjacent_pos = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                for direction in adjacent_pos {
                    let new_i = i as i32 + direction.0;
                    let new_j = j as i32 + direction.1;

                    if (0..(BOARD_SIZE as i32)).contains(&new_i)
                        && (0..(BOARD_SIZE as i32)).contains(&new_j)
                    {
                        if self.board[new_i as usize][new_j as usize] == self.board[i][j] {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score: {}\n\r", self.score)?;

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                write!(f, "{} ", self.board[i][j])?;
                // out_string += " ";
            }
            if i != BOARD_SIZE - 1 {
                write!(f, "\n\r")?;
                // out_string += "\n";
            }
        }
        Ok(())
    }
}
