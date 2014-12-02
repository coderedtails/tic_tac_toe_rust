use std::cell::RefCell;
use io::IO;

pub struct CliSpy {
   pub last: RefCell<Vec<String>>
}

pub fn new() -> CliSpy {
    CliSpy { last: RefCell::new(Vec::new()) }
}

impl IO for CliSpy {
  fn print(&self, line: String) {
    self.last.borrow_mut().push(line);
  }

  fn read(&self) -> String {
      "sentinel value".to_string()
  }
}

impl CliSpy {
    pub fn last_line(&mut self) -> Option<String> {
        self.last.borrow_mut().pop()
  }
}
