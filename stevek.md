2trait Player {
    fn go();
}

struct Human;
struct Ai;

impl Player for Human {
    fn go() { println!("go human!"); }
}

impl Player for Ai {
    fn go() { println!("go AI!"); }
}

struct Game<'a> {
    p1: Box<Player + 'a>,
    p2: Box<Player + 'a>,
}

impl Game<'a> {
   fn go(&self) {
      self.p1.go();
      self.p2.go();
   }
}

fn main() {
    let h = box Human;
    let a = box Ai;
    
    let game = Game { p1: h, p2: a };
    game.go();
}

