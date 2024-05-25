#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    InProgress,
    WhiteMated,
    BlackMated,
    Stalemate,
    Repetition,
    FiftyMoveRule,
    InsufficientMaterial,
    WhiteTimeout,
    BlackTimeout,
}


impl GameResult {
    pub fn is_terminal(self) -> bool {
        self != GameResult::InProgress
    }
}
