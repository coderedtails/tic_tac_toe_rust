use std::cell::RefCell;
use io::Printer;

pub struct CliSpy {
   pub last: RefCell<Vec<String>>
}

pub fn new() -> CliSpy {
    CliSpy { last: RefCell::new(Vec::new()) }
}

impl Printer for CliSpy {
  fn print(&self, line: String) {
    self.last.borrow_mut().push(line);
  }
}

impl CliSpy {
    pub fn last_line(&mut self) -> Option<String> {
        self.last.borrow_mut().pop()
  }
}
