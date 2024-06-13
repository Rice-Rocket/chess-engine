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
./chessatron help
```

## Compiling from Source

- Install rust as instructed [here](https://www.rust-lang.org/tools/install).
- Run the following commands from the terminal in a directory of your choosing: 

```sh
# Clone the source code into `./chessatron/`
git clone https://github.com/Rice-Rocket/chess-engine.git ./chessatron
# Enter the folder with source code
cd chessatron
# Compile the code in release mode
cargo build --release
# Run the binary and show the help menu
cargo run --release -- help
```
