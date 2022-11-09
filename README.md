# Brainfuck compiler in Rust.
A brainfuck compiler written in Rust.

## This project has two components
- Interpreter
    - The interpreter lazily parses the input each character at a time. To invoke the interpreter run passing the source `brainfuck` code in the argument. 
    ```
    cargo run -- hello_world.bf
    ```
- Compiler
    - The compiler converts the `brainfuck` code into `Rust` code into the `output` directory. To invoke the compiler pass `--compile` in the argument.
    ```
    cargo run -- hello_world.bf --compile
    ```

> Note: `console` is added as dependency in `Cargo.toml` to achieve the functionality to `get_char`(',' in brainduck). `std::io::stdin()` doesn't allow to read a single character without the `ENTER` button pressed.

