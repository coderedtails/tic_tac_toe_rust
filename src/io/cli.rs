use io::Printer;
use io::Reader;

pub struct Cli;

impl Printer for Cli {
    fn print(&self, line: String) {
        println!("{}", line)
    }
}

