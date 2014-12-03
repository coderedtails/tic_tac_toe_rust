use std::cell::RefCell;
use io::IO;

pub struct CliSpy {
   pub last: RefCell<Vec<String>>,
   pub moves: RefCell<Vec<String>>
}

pub fn new() -> CliSpy {
    let moves = vec!["1".to_string()];
    new_with_moves(moves)
}

pub fn new_with_moves(moves: Vec<String>) -> CliSpy {
    CliSpy {
              last: RefCell::new(Vec::new()),
              moves: RefCell::new(moves),
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
