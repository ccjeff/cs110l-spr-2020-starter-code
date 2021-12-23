use std::env;
use std::process;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()

struct WC {
    pub word_count : usize,
    pub line_count : usize,
    pub char_count : usize,
}

impl WC {
    pub fn new() -> Self {
        WC {
            word_count : 0,
            line_count : 0,
            char_count : 0,
        }
    }
}

fn read_file_lines(filename: &String) -> Result<WC, io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut result : WC = WC::new();
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                result.line_count += 1;
                result.char_count += line.len();
                let words : Vec<&str> = line.split_whitespace().collect();
                result.word_count += words.len();
            },
            Err(err) => return Err(err),
        }
    }
    // for the \n at the end of the file
    result.char_count += result.line_count;
    Ok(result)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let res: WC = read_file_lines(filename).unwrap();
    println!("{} {} {}", res.word_count, res.line_count, res.char_count);
}
