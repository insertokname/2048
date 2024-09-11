pub const BOARD_SIZE: usize = 4;

pub struct Board {
    pub board: [[u32; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Board {
        let board = Board {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
        };
        board
    }

    fn shift_horizontal(&mut self, range: impl Clone + DoubleEndedIterator<Item = usize>) {
        for i in 0..BOARD_SIZE {
            let mut stack = [0; BOARD_SIZE];
            let mut stack_height = 0;

            for j in range.clone() {
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
                        stack_height -= 2;
                    } else {
                        self.board[i][output_index] = stack[stack_height - 1];
                        stack_height -= 1;
                    }
                }
            }
        }
    }

    fn shift_vertical(&mut self, range: impl Clone + DoubleEndedIterator<Item = usize>) {
        for i in 0..BOARD_SIZE {
            let mut stack = [0; BOARD_SIZE];
            let mut stack_height = 0;

            for j in range.clone() {
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
                        stack_height -= 2;
                    } else {
                        self.board[output_index][i] = stack[stack_height - 1];
                        stack_height -= 1;
                    }
                }
            }
        }
    }

    pub fn shift_right(&mut self) {
        self.shift_horizontal(0..BOARD_SIZE)
    }

    pub fn shift_left(&mut self) {
        self.shift_horizontal((0..BOARD_SIZE).rev())
    }

    pub fn shift_up(&mut self) {
        self.shift_vertical((0..BOARD_SIZE).rev())
    }

    pub fn shift_down(&mut self) {
        self.shift_vertical(0..BOARD_SIZE)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_string = String::new();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                out_string += &self.board[i][j].to_string();
                out_string += " ";
            }
            if i != BOARD_SIZE - 1 {
                out_string += "\n";
            }
        }
        write!(f, "{out_string}")
    }
}
