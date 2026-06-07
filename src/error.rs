use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EngineError {
    #[error("Action not allowed during the current game phase.")]
    InvalidPhase,
    #[error("It is not player {0}'s turn.")]
    NotYourTurn(usize),
    #[error("Player {0} attempted to discard a tile they do not hold.")]
    TileNotFound(usize),
    #[error("Invalid meld call: {0}")]
    InvalidCall(String),
    #[error("Cannot declare Ron: Player is in Furiten.")]
    Furiten,
    #[error("Internal Engine Error: {0}")]
    Internal(String),
}

pub type EngineResult<T> = Result<T, EngineError>;
