pub(crate) const MAIN_OPENING: &str = r#"
fn main() {
"#;

pub(crate) const MAIN_ENDING: &str = r#"

println!();
}
"#;

pub(crate) const INITIALIZE_VARIABLES: &str = r#"
let mut buf = vec![0_i32];
let mut pointer = 0;
"#;

pub(crate) const HELPER_FUNCS: &str = r#"

fn fill_array(buf: &mut Vec<i32>, pointer: usize) {
    while buf.len() <= pointer {
        buf.push(0);
    }
}

fn increment_value(buf: &mut Vec<i32>, pointer: usize) {
    fill_array(buf, pointer);
    buf[pointer] += 1;
}

fn decrement_value(buf: &mut Vec<i32>, pointer: usize) {
    fill_array(buf, pointer);
    buf[pointer] -= 1;
}

fn put_char(buf: &Vec<i32>, pointer: usize) {
    print!("{}", (buf[pointer] as u8) as char);
}

fn get_char(buf: &mut Vec<i32>, pointer: usize) {
    fill_array(buf, pointer);
    println!();
    println!("Waiting for user input!");
    let term = console::Term::stdout();
    let ch = term.read_char().unwrap();
    buf[pointer] = ch as i32;
}

"#;

pub(crate) fn get_cargo_toml_content(name: &str) -> String {
    let mut content = String::with_capacity(100);
    content.push_str("[package]");
    content.push('\n');
    let name = format!("name = \"{name}\"");
    content.push_str(&name);
    content.push('\n');
    content.push_str("version = \"0.1.0\"");
    content.push('\n');
    content.push_str("edition = \"2021\"");
    content.push('\n');
    content.push('\n');
    content.push_str("[dependencies]");
    content.push('\n');
    content.push_str("console = \"0.15.2\"");
    content.push('\n');
    content.push('\n');
    content.push_str("[[bin]]");
    content.push('\n');
    content.push_str(&name);
    content.push('\n');
    content.push('\n');
    content
}

pub const HELP_TEXT: &str = "
To compile a brainfuck source file:
    brainfuck-rs --compile <source_file_name>.bf

To interpret a brainfuck source file:
    brainfuck-rs <source_file_name>.bf
";

pub(crate) const WHILE_LOOP_START: &str = "while buf[pointer] != 0 {\n";
pub(crate) fn get_increment_value_text(num: usize) -> String {
    format!("(0..{num}).for_each(|_| increment_value(&mut buf, pointer));\n")
}
pub(crate) fn get_decrement_value_text(num: usize) -> String {
    format!("(0..{num}).for_each(|_| decrement_value(&mut buf, pointer));\n")
}
pub(crate) fn get_move_right_text(num: usize) -> String {
    format!("(0..{num}).for_each(|_| pointer += 1);\n")
}
pub(crate) fn get_move_left_text(num: usize) -> String {
    format!("(0..{num}).for_each(|_| pointer -= 1);\n")
}
pub(crate) fn get_put_char_text(num: usize) -> String {
    format!("(0..{num}).for_each(|_| put_char(&buf, pointer));\n")
}
pub(crate) fn get_get_char_text(num: usize) -> String {
    format!("(0..{num}).for_each(|_| get_char(&mut buf, pointer));\n")
}
