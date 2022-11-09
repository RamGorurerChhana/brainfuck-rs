use brainfuck_rs::*;
use std::env;

enum Mode {
    Interpret,
    Compile,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Invalid command given!");
        eprintln!("{}", HELP_TEXT);
        return;
    }
    let mut mode = Mode::Interpret;
    let mut source_file = "";
    if args[1] == "--compile" {
        mode = Mode::Compile;
    } else if args[1].ends_with(".bf") {
        source_file = args[1].as_str();
    } else {
        eprintln!("Unrecognized input: {}", args[1]);
        eprintln!("{}", HELP_TEXT);
        return;
    }
    if args.len() > 2 {
        if args[2] == "--compile" {
            mode = Mode::Compile;
        } else if args[2].ends_with(".bf") {
            source_file = args[2].as_str();
        } else {
            eprintln!("Unrecognized input: {}", args[1]);
            eprintln!("{}", HELP_TEXT);
            return;
        }
    }

    if source_file == "" {
        eprintln!("Invalid command given!");
        eprintln!("{}", HELP_TEXT);
        return;
    }
    let input = read_source_file(source_file);
    match mode {
        Mode::Interpret => Interpreter::new(input).run(),
        Mode::Compile => {
            let binary = source_file.strip_suffix(".bf").unwrap();
            Compiler::new(input, binary.to_string()).compile();
        }
    }
}
