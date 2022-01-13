# touchtype
A dead simple training program for the home row keys.

The program will continuously request the player to type a random character from the home row, until the player types the wrong key. The player's score is the number of characters typed correctly.

## Usage

To play the game, use `cargo run`.

Since this project uses [termios](https://crates.io/crates/termios) to configure player input, only Unix-like operating systems are supported.
