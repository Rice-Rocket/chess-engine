// #[derive(PartialEq, Clone, Copy)]
// pub enum Team {
//     White,
//     Black,
// }

// impl Team {
//     // For use with piece values
//     pub fn piece(self) -> u8 {
//         if self == Team::White { 8 } else { 16 }
//     }
//     // For indexing
//     pub fn index(self) -> usize {
//         if self == Team::White { 0 } else { 1 }
//     }
// }

// impl From<u8> for Team {
//     fn from(value: u8) -> Self {
//         if value == 8 { Team::White } else { Team::Black }
//     }
// }

// impl From<usize> for Team {
//     fn from(value: usize) -> Self {
//         if value == 0 { Team::White } else { Team::Black }
//     }
// }