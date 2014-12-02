use io::Printer;
use core::marker::Marker;
use core::board::Board;

use std::io;

pub struct Display<P> {
    pub cli: P
}

impl<P: Printer> Display<P> {
    pub fn render(&self, board: Board) {
        let lines = render(board);
        self.cli.print(lines.connect("\n"));
    }

    fn to_int(&self, input: String) -> uint {
        let raw: Option<uint> = from_str(input.as_slice().trim());

        match raw {
            Some(number) => number,
            None => 100,
        }
    }

    pub fn request_move(&self) -> uint {
        println!("Choose move");
        let input = io::stdin().read_line().ok().expect("Failed to read line");
        self.to_int(input)
    }
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
    let inner = player.to_string(idx);
    format!("[{}]", inner)
}
