use io::IO;
use core::marker::Marker;
use core::board::Board;
use core::slot::Slot;
use game::game_mode::GameMode;

use ansi_term::Colour::{Red, Blue, White};

pub struct Display<P> {
    pub cli: P,
    pub use_colour: bool,
}

impl<P: IO> Display<P> {
    pub fn draw(&self, board: Board) {
        self.clear_screen();
        self.draw_board(board);
    }

   fn clear_screen(&self) {
        self.cli.print("\x1b[H\x1b[2J");
    }

    fn draw_board(&self, board: Board) {
        let lines = self.render(board);
        self.cli.print(lines.connect("\n").as_slice());
    }

    pub fn announce_winner(&self, winner: Marker) {
       self.cli.print(self.winner_line(winner).as_slice())
    }

    fn winner_line(&self, winner: Marker) -> String {
        format!("The winner was {}", winner.to_string())
    }


    pub fn announce_draw(&self) {
        self.cli.print("There was a draw");
    }

    pub fn show_options<'a>(&self, modes: &[GameMode<'a>, ..4]) {
        self.clear_screen();
        for (idx, mode) in modes.iter().enumerate() {
            self.show_option(mode, idx);
        }
    }

    pub fn show_option<'a>(&self, modes: &GameMode<'a>, idx: uint) {
        let line = format!("{}: {}", idx, modes.opponents_line());
        self.cli.print(line.as_slice());
    }

    pub fn request_mode(&self) -> uint {
        self.read_int_with_output("Choose game mode:")
    }

    pub fn request_move(&self) -> uint {
        self.read_int_with_output("Choose move")
    }

    fn read_int_with_output(&self, output: &str) -> uint {
        self.cli.print(output);
        let input = self.cli.read();
        self.to_int(input)
    }

    fn to_int(&self, input: String) -> uint {
        let raw: Option<uint> = from_str(input.as_slice().trim());
        match raw {
            Some(number) => number,
            None => 100,
        }
    }

    pub fn request_rematch(&self) -> bool {
        self.cli.print("Want a rematch? (Y/N)");
        let input = self.cli.read();
        match input.as_slice().trim() {
            "Y" => true,
            _ => false,
        }
    }

    fn render(&self, board: Board) -> Vec<String> {
        board.rows().iter().map(|x| self.render_line(*x)).collect()
    }

    fn render_line(&self, line: &[Slot]) -> String {
        line.iter().map(|x| self.render_slot(*x)).collect::<Vec<String>>().concat()
    }

    pub fn render_slot(&self, slot: Slot) -> String {
        let inner = if self.use_colour  {
            render_colour_slot(slot)
        } else {
            slot.printable()
        };
        format!("[{}]", inner)
    }
}

pub fn render_colour_slot(slot: Slot) -> String {
    let printable = slot.printable();
    let inner = printable.as_slice();
    let ansi = match slot {
        Slot::Placed(Marker::X) => Red.paint(inner),
        Slot::Placed(Marker::O) => Blue.paint(inner),
        Slot::Move(_) => White.paint(inner),
    };
    ansi.to_string()
}
