use std::cell::Cell;
use io::Printer;
use io::Reader;

pub struct CliSpy {
    lastLine: Cell<String>
}

pub fn new() -> CliSpy {
    CliSpy{ lastLine: Cell::new("".to_string()) }
}

impl Printer for CliSpy {
    fn print(&self, line: String) {
        self.lastLine.set(line);
    }
}

impl CliSpy {
    fn last_line(&self) -> String {
       self.lastLine.get()
    }
}


