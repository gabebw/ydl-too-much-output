# Too much output overwhelms STDOUT

A minimal example demonstrating how very large output (for example, JSON from
Youtube-DL) can overwhelm internal buffers, hanging the program forever.

To see it hang,comment out [these three
lines](https://github.com/gabebw/ydl-too-much-output/blob/main/src/main.rs#L25-L27).

## Running the example

    cargo run
