# Advent of Code, 2015

This project contains my solutions for the 2015 Advent of Code challenges.

## Usage

To run a solution for a puzzle, run: 

`cargo run -- <DAY> <PART>`

Where `<DAY>` is the day number (1-25) and part is 1 or 2.

You can also run `cargo test day<N>` to run the unit tests for a given day.
These usually cover the sample cases for the puzzles, occasionally among other things.

## Organization

Each day's puzzle has it's solution code in `/src/solutions/dayN.rs`. For more complex puzzles
these may be just entry points/indexes for the module.  Common code used across the solutions
is put in `/src/common/*`.