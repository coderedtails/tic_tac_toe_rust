use io::IO;
use core::marker::Marker;
use core::board::Board;
use players::game_mode::GameMode;

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
        match winner  {
            Marker::Empty => panic!("Empty can not be the winner"),
            _ => self.cli.print(self.winner_line(winner).as_slice()),
        }
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

    fn winner_line(&self, winner: Marker) -> String {
        format!("The winner was {}", winner.to_string())
    }

    fn render(&self, board: Board) -> Vec<String> {
        let rows = board.rows();
        let mut result = Vec::new();
        let mut offset = 0u;
        for row in rows.iter() {
            let line = self.render_line(*row, offset);
            offset += 3;
            result.push(line);
        }
        result
    }

    fn render_line(&self, line: &[Marker], offset: uint) -> String {
        line.iter().enumerate()
            .map(|(idx, player)| self.render_cell((idx+offset, player)))
            .collect::<Vec<String>>().concat()
    }

    pub fn render_cell(&self, elements: (uint, &Marker)) -> String {
        let (idx, player) = elements;
        let inner = if self.use_colour  {
            self.render_colour_cell(idx, *player)
        } else {
            player.as_string(idx)
        };
        format!("[{}]", inner)
    }

    pub fn render_colour_cell(&self, idx: uint, player: Marker) -> String {
        let inner = player.as_string(idx);
        match player {
            Marker::X => format!("{}", Red.paint(inner.as_slice())),
            Marker::O => format!("{}", Blue.paint(inner.as_slice())),
            Marker::Empty => format!("{}", White.paint(inner.as_slice())),
        }
    }
}
