use io::IO;
use std::cell::RefCell;

pub struct CliSpy {
   pub printed_lines: RefCell<Vec<String>>,
   pub moves: RefCell<Vec<String>>
}

pub fn new() -> CliSpy {
    new_with_input("")
}

pub fn new_with_input(input: &str) -> CliSpy {
    let split: Vec<String> = input.split(',').map(|x| x.to_string()).collect();
    CliSpy {
        printed_lines: RefCell::new(Vec::new()),
        moves: RefCell::new(split),
    }
}

impl IO for CliSpy {
  fn print(&self, line: &str) {
    self.printed_lines.borrow_mut().push(line.to_string());
  }

  fn read(&self) -> String {
      match self.moves.borrow_mut().pop() {
          Some(n) => n,
          None => panic!("There was nothing to read!")
      }
  }
}

impl CliSpy {
    pub fn last_line(&mut self) -> Option<String> {
        self.printed_lines.borrow_mut().pop()
  }
}
