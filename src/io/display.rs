use io::IO;
use core::marker::Marker;
use core::board::Board;

pub struct Display<P> {
    pub cli: P
}

impl<P: IO> Display<P> {
    pub fn render(&self, board: Board) {
        self.clear_screen();
        self.draw_board(board);
    }

    fn clear_screen(&self) {
        self.cli.print("\x1b[H\x1b[2J".to_string());
    }

    fn draw_board(&self, board: Board) {
        let lines = render(board);
        self.cli.print(lines.connect("\n"));
    }

    pub fn request_move(&self) -> uint {
        self.cli.print("Choose move".to_string());
        let input = self.cli.read();
        self.to_int(input)
    }

    pub fn announce_winner(&self, winner: Marker) {
        match winner  {
            Marker::Empty => panic!("Empty can not be the winner"),
            _ => self.cli.print(winner_line(winner)),
        }
    }

    pub fn announce_draw(&self) {
        self.cli.print("There was a draw".to_string());
    }

    fn to_int(&self, input: String) -> uint {
        let raw: Option<uint> = from_str(input.as_slice().trim());

        match raw {
            Some(number) => number,
            None => 100,
        }
    }
}

fn winner_line(winner: Marker) -> String {
    format!("The winner was {}", winner.to_string())
}

pub fn render(board: Board) -> Vec<String> {
    let rows = board.rows();
    let mut result = Vec::new();
    let mut offset = 0u;
    for row in rows.iter() {
        let line = render_line(*row, offset);
        offset += 3;
        result.push(line);
    }
    result
}

pub fn render_line(line: &[Marker], offset: uint) -> String {
    line.iter().enumerate()
               .map(|(idx, player)| render_cell((idx+offset, player)))
               .collect::<Vec<String>>()
               .concat()
}

pub fn render_cell(elements: (uint, &Marker)) -> String {
    let (idx, player) = elements;
    let inner = player.as_string(idx);
    format!("[{}]", inner)
}
