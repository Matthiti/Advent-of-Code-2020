use std::io;
use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use std::error::Error;
use std::str::FromStr;
use core::fmt;

#[derive(Debug)]
struct ParseError {
    message: String
}

impl ParseError {
    fn new(message: String) -> Self {
        ParseError {message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub fn parse_file<T: FromStr>(file: &str) -> Result<Vec<T>, Box<dyn Error>>  {
    let mut result = vec![];
    for line in read_lines(file)? {
        if let Ok(line) = line {
            if let Ok(value) = line.parse() {
                result.push(value);
            } else {
                return Err(Box::new(ParseError::new(String::from("Cannot parse value"))));
            }
        }
    }
    Ok(result)
}

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}