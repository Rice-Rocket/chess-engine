# Chess Engine In Rust (V2)

## Description

A Chess Engine built in Rust using [Bevy ECS](https://bevyengine.org/).

### Game Features

- [x] Player vs Player, Player vs Engine, and Engine vs Engine modes.
- [x] Fast pseudo-legal move generation using magic bitboards.
- [x] Legal moves generation that accounts for pins and checks.
- [x] Castling and En Passant (AKA the bane of my existence).

### AI Features

#### Search

- [x] Iterative Deepening
- [x] Alpha-beta pruning with move ordering
- [x] Transposition Tables
- [x] Quiescence Search
- [x] Depth Reductions
- [x] Search Extensions

#### Evaluation

- [x] Tapered Evaluation
- [x] Material Bonuses
- [x] Piece Square Tables
- [x] Imbalance Evaluation
- [x] Mobility Evaluation


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