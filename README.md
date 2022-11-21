# rustfuck
A simple brainfuck interpreter written in Rust.

# How to get started
1. Get `rustup` and `cargo`
2. Install the latest stable Rust toolchain
3. Compile the interpreter by running `cargo build --release`
4. Put the compiled binary located in `target/release/` in one of your folders with binaries
5. Run the program through a terminal and pass a path to a brainfuck file via command line args.

# Example
Assuming `rustfuck` is in one of your PATH directories:
```
rustfuck examples/hello.bf
```
