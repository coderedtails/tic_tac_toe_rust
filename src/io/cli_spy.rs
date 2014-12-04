use std::cell::RefCell;
use io::IO;

pub struct CliSpy {
   pub last: RefCell<Vec<String>>,
   pub moves: RefCell<Vec<String>>
}

pub fn new() -> CliSpy {
    new_with_input("1")
}

pub fn new_with_input(input: &str) -> CliSpy {
    let split: Vec<String> = input.split(',').map(|x| x.to_string()).collect();
    CliSpy {
        last: RefCell::new(Vec::new()),
        moves: RefCell::new(split),
    }
}

impl IO for CliSpy {
  fn print(&self, line: &str) {
    self.last.borrow_mut().push(line.to_string());
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
        self.last.borrow_mut().pop()
  }
}
