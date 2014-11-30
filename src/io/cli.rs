use io::Printer;

pub struct Cli;

pub fn new() -> Cli {
    Cli
}

impl Printer for Cli {
    fn print(&self, line: String) {
        println!("{}", line)
    }
}

