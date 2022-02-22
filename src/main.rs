use std::collections::HashMap;
use std::io;

type Cell = u8;
const ARRAY_SIZE: usize = 30000;

struct Interpreter {
    array: [Cell; ARRAY_SIZE],
    current: usize
}

impl Interpreter {
    fn new() -> Self {
        Self {
            array: [0; ARRAY_SIZE],
            current: 0
        }
    }

    fn exec(&mut self, code: &str) -> Result<(), String> {
        let code_len = code.chars().count();
        let mut brackets: HashMap<usize, usize> = HashMap::new();
        let mut stack: Vec<usize> = vec![];
        for i in 0..code_len {
            let c = code.chars().nth(i).unwrap();
            match c {
                '[' => stack.push(i),
                ']' => {
                    let open_i = match stack.pop(){
                        Some(value) => value,
                        None => return Err(format!("close brackets without open one\nchar={}", i))
                    };
                    brackets.insert(open_i, i);
                    brackets.insert(i, open_i);
                },
                _ => {}
            }
        }
        if !stack.is_empty() {
            return Err(format!("not all brackets are closed"))
        }

        let mut i: usize = 0;
        while i < code_len {
            let c = code.chars().nth(i).unwrap();
            let value = self.array[self.current];
            match c {
                '+' if value == Cell::MAX => return Err(format!("buffer overflow\ncell={}\nchar={}", self.current, i)),
                '+' => self.array[self.current] += 1,
                '-' if value == Cell::MIN => return Err(format!("buffer underflow\ncell={}\nchar={}", self.current, i)),
                '-' => self.array[self.current] -= 1,
                '>' if self.current >= ARRAY_SIZE + 1 => return Err(format!("array end reached\ncell={}\nchar={}", self.current, i)),
                '>' => self.current += 1,
                '<' if self.current == 0 => return Err(format!("array begin reached\ncell={}\nchar={}", self.current, i)),
                '<' => self.current -= 1,
                '.' => print!("{}", self.array[self.current] as char),
                ',' => {
                    let mut inp = String::new();
                    match std::io::stdin().read_line(&mut inp) {
                        Ok(_) => {},
                        Err(e) => return Err(format!("can't read from stdio error={:?}", e))
                    }
                    // TODO: make it write byte from input to current
                },
                '['  => {
                    if self.array[self.current] == 0 {
                        i = *brackets.get(&i).unwrap();
                        continue
                    }
                },
                ']' => {
                    if self.array[self.current] != 0 {
                        i = *brackets.get(&i).unwrap();
                        continue
                    }
                },
                _ => {}
            }
            i += 1;
        }
        Ok(())
    }
}


fn main() {
    let mut itp = Interpreter::new();
    itp.exec("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.[.].");
    println!();
}
