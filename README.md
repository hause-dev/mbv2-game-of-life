# The Game of Life on the micro::bit v2

## Description

This is a basic Rust implementation for John Conway's "Game of Life" running on the
micro::bit v2 microcontroller's 5x5 LED matrix. The program rules are as follows:

- The game starts with a random world state and runs at 10 frames per second.
- As long as the A button is held, the board will generate a random world state every frame.
- Otherwise, if the B button is held, the board will invert its state and block the B button
  for 5 frames (or half a second).
- If no button is held, the board will take a normal game of life step.
- If the game of life terminates (i.e. all cells are dead), the board will wait either for
  a button press or for 5 frames, upon which a random world state is generated and the game
  begins once again.

## Build and run

- To build the application without flashing: `cargo build --release` from repo root
- To build the application and flash to the micro::bit: `cargo embed --release` from repo root

## Implementation details

TBD

## Attribution

All code in the `life.rs` file was provided by [Bart Massey](https://github.com/BartMassey).
