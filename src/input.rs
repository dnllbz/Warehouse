use std::io::{stdin, stdout, Write};

pub fn input_with_prompt(ptr: &mut String, text: &str) {
    print!("{}", text);
    ptr.clear();
    let _ = stdout().flush();
    let _ = stdin().read_line(ptr);
}