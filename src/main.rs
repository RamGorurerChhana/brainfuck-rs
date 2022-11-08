use brainfuck_rs::Interpreter;

fn main() {
    let input = "+++++++++[>++++++++>+++++++++++>++++>++++++++++++<<<<-]>.>++.>>..+++.<----.>+++++.<<+++.---.>>--.<<.>+.";
    Interpreter::new(input.to_string()).run();
}
