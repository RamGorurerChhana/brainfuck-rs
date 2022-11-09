use crate::file_handler::*;
use crate::templates::*;
use crate::VALID_CHARS;

pub struct Compiler {
    input: Vec<char>,
    cursor: usize,
    binary: String,
}

impl Compiler {
    pub fn new(input: String, binary: String) -> Self {
        Self {
            input: input
                .chars()
                .filter(|ch| VALID_CHARS.contains(ch))
                .collect(),
            cursor: 0,
            binary,
        }
    }

    pub fn compile(&mut self) {
        let mut file = initialize_output_dir(&self.binary);
        write_to_file(&mut file, MAIN_OPENING);
        write_to_file(&mut file, INITIALIZE_VARIABLES);
        loop {
            if self.cursor >= self.input.len() {
                break;
            }
            let mut count = 0;
            let ch = self.input[self.cursor];
            count += 1;
            self.cursor += 1;
            while self.cursor < self.input.len() && self.input[self.cursor] == ch {
                count += 1;
                self.cursor += 1;
            }

            match ch {
                '+' => write_to_file(&mut file, get_increment_value_text(count).as_str()),
                '-' => write_to_file(&mut file, get_decrement_value_text(count).as_str()),
                '>' => write_to_file(&mut file, get_move_right_text(count).as_str()),
                '<' => write_to_file(&mut file, get_move_left_text(count).as_str()),
                '[' => (0..count).for_each(|_| write_to_file(&mut file, WHILE_LOOP_START)),
                ']' => (0..count).for_each(|_| write_to_file(&mut file, "}\n")),
                '.' => write_to_file(&mut file, get_put_char_text(count).as_str()),
                ',' => write_to_file(&mut file, get_get_char_text(count).as_str()),
                _ => {}
            }
        }

        write_to_file(&mut file, MAIN_ENDING);
        write_to_file(&mut file, HELPER_FUNCS);
        finish_output(&mut file);
    }
}
