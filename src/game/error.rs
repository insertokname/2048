use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError{
    #[error("Error: Game is dead!")]
    GameDeadError,
    #[error("Error: Couldn't play a move!\n\rGot error: {0}")]
    PlayError(Box<dyn std::error::Error>),
    #[error("Error: Couldn't generate a space!\n\rGot error: {0}",)]
    GenerateError(Box<dyn std::error::Error>),
}