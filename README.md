# Chess Engine (V2)

## Description

A Chess Engine built in Rust.

### Features

#### Search

- Iterative Deepening
- Aspiration Windows
- Transposition Tables
- Move Ordering
    - Internal Iterative Deepening
    - History Heuristic
    - Killer Heuristic
    - MVV/LVA
- Extensions
    - Check Extensions
- Pruning
    - Alpha-beta Pruning
- Reductions
    - Late Move Reductions

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
