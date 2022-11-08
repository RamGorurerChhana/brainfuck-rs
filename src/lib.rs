use console::Term;

/// Interpreter struct controls the execution of the programs
/// It reads input and executes the commands from the input.
/// ```
/// use brainfuck_rs::Interpreter;
/// let input = "++++++++++[>++++++++++<-]>++++.+.".to_string();
/// Interpreter::new(input).run();
/// ```
#[derive(Debug)]
pub struct Interpreter {
    // holds the entire input
    input: Vec<char>,
    // holds the position of the cursor on the input stream
    cursor: usize,
    // holds the position of the pointer to the current cell in the array
    pointer: usize,
    // array for numeric operations
    array: Vec<i32>,
}

const VALID_CHARS: [char; 8] = ['>', '<', '+', '-', ',', '.', '[', ']'];
impl Interpreter {
    // creates a new instance of the Interpreter
    pub fn new(input: String) -> Self {
        Self {
            input: input
                .chars()
                .filter(|ch| VALID_CHARS.contains(ch))
                .collect(),
            cursor: 0,
            pointer: 0,
            array: vec![],
        }
    }

    // runs the interpreter and and process the input stream
    // Note: this method is public but takes ownership of self.
    // So that same interpreter insatnce cannot be used twice.
    pub fn run(mut self) {
        self.execute();
    }

    // process the input stream and execute the commands
    fn execute(&mut self) {
        while self.cursor < self.input.len() {
            let ch = self.input[self.cursor];
            match ch {
                '>' => self.move_right(),
                '<' => self.move_left(),
                '+' => self.increment(),
                '-' => self.decrement(),
                '.' => self.put_char(),
                ',' => self.get_char(),
                // start a loop
                '[' => self.execute_loop(self.cursor),
                // end the loop and return back to the caller
                ']' => return,
                // for all other characters do nothing
                _ => {}
            };
            self.cursor += 1;
        }
        // empty println to flush the output before terminating the program
        println!();
    }

    // increment pointer
    fn move_right(&mut self) {
        self.pointer += 1;
    }

    // decrement pointer
    fn move_left(&mut self) {
        self.pointer -= 1;
    }
    // if the array length is less than pointer position
    // then fill in the blank spots with 0
    fn fill_array(&mut self) {
        while self.array.len() <= self.pointer {
            self.array.push(0);
        }
    }

    // increment value
    fn increment(&mut self) {
        self.fill_array();
        self.array[self.pointer] += 1;
    }

    // decrement value
    fn decrement(&mut self) {
        self.fill_array();
        self.array[self.pointer] -= 1;
    }

    // print a character
    fn put_char(&mut self) {
        self.fill_array();
        print!("{}", (self.array[self.pointer] as u8) as char);
    }

    // get user input
    fn get_char(&mut self) {
        // empty line to flush previous output
        println!();
        println!("Waiting for user input!");
        let term = Term::stdout();
        let ch = term.read_char().unwrap();
        self.fill_array();
        self.array[self.pointer] = ch as i32;
    }

    // begin executing a loop
    fn execute_loop(&mut self, start: usize) {
        // skip the loop execution if the current value is 0
        // simply advance the cursor until mathcing ']' received
        self.fill_array();
        if self.array[self.pointer] == 0 {
            self.cursor += 1;
            while self.input[self.cursor] != ']' {
                if self.cursor >= self.input.len() {
                    panic!("unbalanced bracket!");
                }
                if self.input[self.cursor] == '[' {
                    self.execute_loop(self.cursor);
                }
                self.cursor += 1;
            }
            return;
        }
        loop {
            self.cursor = start + 1;
            self.execute();
            // current char must be ']' otherwise unbalanced bracket present
            if self.cursor >= self.input.len() || self.input[self.cursor] != ']' {
                panic!("unbalanced bracket!");
            }
            self.fill_array();
            // terminate the loop if current char is zero
            if self.array[self.pointer] == 0 {
                return;
            }
        }
    }
}
