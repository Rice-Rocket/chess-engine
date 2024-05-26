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

    pub fn is_draw(self) -> bool {
        matches!(self, GameResult::Stalemate | GameResult::Repetition | GameResult::FiftyMoveRule | GameResult::InsufficientMaterial)
    }

    pub fn is_white_win(self) -> bool {
        matches!(self, GameResult::BlackMated | GameResult::BlackTimeout)
    }

    pub fn is_black_win(self) -> bool {
        matches!(self, GameResult::WhiteMated | GameResult::WhiteTimeout)
    }
}
