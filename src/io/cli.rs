use io::IO;
use std::io;

pub struct Cli;

pub fn new() -> Cli {
    Cli
}

impl IO for Cli {
    fn print(&self, line: String) {
        println!("{}", line)
    }

    fn read(&self) -> String {
        io::stdin().read_line().ok().expect("Failed to read line")
    }
}

