# Chess-a-tron (V2)

## Description

A relatively powerful chess engine built in rust.

### Features

#### Search

- Iterative Deepening
- Aspiration Windows
- Transposition Tables
    - Depth-preferred Replacement Strategy
- Move Ordering
    - Internal Iterative Deepening
    - History Heuristic
    - Killer Heuristic
    - MVV/LVA
- Selectivity
    - Extensions
        - Check Extensions
    - Pruning
        - Alpha-beta Pruning (fail-high / beta cutoff)
    - Reductions
        - Late Move Reductions
    - Quiescence Search

#### Evaluation

- Tapered Evaluation
- Material
    - Point Values
    - Bishop Pair
    - Imbalance Tables
- Piece-Square Tables
- Space
- Mobility
    - Trapped Pieces
    - Rooks on (Semi) Open Files
- Outposts
- Pawn Structure
    - Supported Pawn
    - Backward Pawn
    - Doubled Pawn
    - Isolated Pawn
    - Blocked Pawn
    - Phalanx
    - Connected Pawns
    - Passed Pawn
    - King Proximity
- Threats
    - Queen Infiltration
    - Restricted Squares
    - Slider/Knight on Queen
    - Weak Pieces
    - Hanging Pieces
- King Safety
    - Attacking King Ring
    - Pawn Shelter
    - Pawn Storm
    - Checks
    - Square Control

## Installation

Download the executable from the 'releases' section and run it with:

```sh
# Show the help menu
chessatron help
```

## Compiling from Source

- Install Rust as instructed [Here](https://www.rust-lang.org/tools/install).
- Download and unzip the source code.
- Open a terminal in the directory just created by unzipping the file.
- Compile and run the code using the following commands: 

```sh
# Compile the code in release mode
cargo build --release
# Run the code in release mode
cargo run --release
```
