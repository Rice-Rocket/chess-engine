pub mod masks;
pub mod bb;


// IDEA: Fuse multiple bitboards together to make a multi bitboard: 
//
// struct MultiBitBoard<const N: usize>([BitBoard; N]);
//
// Allow these bitboards to support counting in binary across all bitboards.
// Contains square will be the extracted number > 0: 
//
// let bb = MultiBitBoard([0; 2]);
// bb.square_mut(18) += 2;  // Or use just support indexing
// assert!(bb.contains_square(18))
// assert_eq!(bb.get_square(18), 2)
//
// Figure out a way to make bitwise operations SIMD stable.
//
// This could be helpful in speeding up some evaluation functions with larger per-square values. 
//
// Under the hood for representing values: 
//
//    1 2 1 0 2 0 3 1
//    v v v v v v v v
// 0b 0 1 0 0 1 0 1 0
// 0b 1 0 1 0 0 0 1 1
