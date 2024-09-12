use thiserror::Error;

#[derive(Error, Debug)]
pub enum BoardError{
    #[error("Error: The shift didn't modify the board!")]
    UnchangedShiftError
}