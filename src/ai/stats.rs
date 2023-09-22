pub struct SearchStatistics {
    pub num_position_evals: u32,
    pub num_cutoffs: i32,
    pub think_time_ms: u32,
    pub num_checks: i32,
    pub num_mates: i32,
    pub num_transpositions: i32,
    pub is_book: bool,
}

impl Default for SearchStatistics {
    fn default() -> Self {
        Self {
            num_position_evals: 0,
            num_cutoffs: 0,
            think_time_ms: 0,
            num_checks: 0,
            num_mates: 0,
            num_transpositions: 0,
            is_book: false,
        }
    }
}