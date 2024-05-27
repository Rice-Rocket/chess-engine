use super::Searcher;

#[derive(Default, Clone, Copy)]
pub struct SearchDiagnostics {
    pub depth_searched: u8,
    pub evaluation: i32,
}

impl SearchDiagnostics {
    pub fn is_mate_score(&self) -> bool {
        self.evaluation.abs() > Searcher::IMMEDIATE_MATE_SCORE - 1000
    }
    
    pub fn moves_till_mate(&self) -> i32 {
        Searcher::IMMEDIATE_MATE_SCORE - self.evaluation.abs() - 1
    }
}
