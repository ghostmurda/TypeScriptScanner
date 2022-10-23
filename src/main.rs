// SCANNER
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod scanner;

fn main() {
    let mut scanner = scanner::Scanner {
        tokens: Vec::new(),
        cur_line: Vec::new(),
        line_number: 0,
        word_buffer: String::new(),
        word_buffer_column: 1,
    };

    if let Ok(lines) = read_lines("./test.ts") {
        for line in lines {
            if let Ok(ip) = line {
                scanner.push_line(&ip);
            }
        }
    }

    println!("scanner: {:#?}", scanner);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
