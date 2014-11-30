use std::cell::RefCell;
use io::Printer;

pub struct CliSpy {
   last: RefCell<String>
}

pub fn new() -> CliSpy {
    CliSpy { last: RefCell::new("".to_string()) }
}

impl Printer for CliSpy {
  fn print(&self, line: String) {
    *self.last.borrow_mut() = line;
  }
}

impl CliSpy {
    pub fn last_line(&self) -> String {
        let result = self.last.borrow_mut().clone();
        result
  }
}
