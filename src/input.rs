use std::io::{stdin, stdout, Write};

pub fn print_input(ptr: &mut String, text: &str) {
    print!("{}", text);
    let _ = stdout().flush();
    let _ = stdin().read_line(ptr);
}