## Conway's Quine
This is a ***1000% extremely*** life-altering tool that simulates Conway's Game of Life in an unusual way. Running it produces a copy of its source code, except the Game of Life board advances by one generation with each run.

The current implementation on the board represents [Gosper's Gun](https://en.wikipedia.org/wiki/Gun_(cellular_automaton)), which is a cellular automaton that oscillates indefinitely, emitting gliders. 

You can rearrange the board by replacing the `x` cells with spaces and vice versa.

## Running this program

To keep this as quine-like as possible, the command to run it is a bit hacky, but it circumvents `src/main.rs` from being overwritten immediately before it can finish running. The following command can be used to run: 

```bash 
cargo run > temp && mv temp src/main.rs
