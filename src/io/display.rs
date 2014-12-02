use io::IO;
use core::marker::Marker;
use core::board::Board;

pub struct Display<P> {
    pub cli: P
}

impl<P: IO> Display<P> {
    pub fn render(&self, board: Board) {
        let lines = render(board);
        self.cli.print(lines.connect("\n"));
    }

    pub fn request_move(&self) -> uint {
        self.cli.print("Choose move".to_string());
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
