use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ICANON, tcsetattr};
use rustyline::Editor;

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
                '+' => self.array[self.current] = value.wrapping_add(1),
                '-' => self.array[self.current] = value.wrapping_sub(1),
                '>' if self.current >= ARRAY_SIZE + 1 => return Err(format!("array end reached\ncell_index={}\nline/char={:?}", self.current, get_line_char(code, i))),
                '>' => self.current += 1,
                '<' if self.current == 0 => return Err(format!("array begin reached\ncell_index={}\nline/char={:?}", self.current, get_line_char(code, i))),
                '<' => self.current -= 1,
                '.' => {
                    print!("{}", self.array[self.current] as char);
                    std::io::stdout().flush().unwrap();
                },
                ',' => {
                    let mut inp: [u8; 1] = [0];
                    let mut stdin = std::io::stdin();
                    let old_termios = Termios::from_fd(0).unwrap();
                    let mut termios = old_termios.clone();
                    termios.c_lflag &= !(ICANON);
                    tcsetattr(0, TCSANOW, &mut termios).unwrap();
                    stdin.read(&mut inp).unwrap();
                    self.array[self.current] = inp[0];
                    tcsetattr(0, TCSANOW, & old_termios).unwrap();
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

fn get_line_char(code: &str, i: usize) -> Option<(usize, usize)> {
    let mut line = 1;
    let mut chr = 0;
    for j in 0..=i {
        let c = match code.chars().nth(j){
            Some(v) => v,
            None => return None
        };
        if c == '\n' {
            line += 1;
            chr = 0;
        } else {
            chr += 1;
        }
    }
    Some((line, chr))
}

fn main() {
    let mut itp = Interpreter::new();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        let mut rl = Editor::<()>::new();
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    match itp.exec(&line) {
                        Ok(_) => println!(),
                        Err(e) => println!("{}", e)
                    }
                },
                Err(_) => break
            }
        }
    } else {
        let filepath = args.get(1).unwrap();
        let code = fs::read_to_string(filepath).expect("can't read file");
        match itp.exec(&code) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }
    }
    println!();
}
