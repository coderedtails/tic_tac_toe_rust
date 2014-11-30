use line::Player;
use board::Board;

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

pub fn render_line(line: &[Player], offset: uint) -> String {
    line.iter().enumerate()
               .map(|(idx, player)| render_cell((idx+offset, player)))
               .collect::<Vec<String>>()
               .concat()
}

pub fn render_cell(elements: (uint, &Player)) -> String {
    let (idx, player) = elements;
    match *player {
        Player::X => "[X]".to_string(),
        Player::O => "[O]".to_string(),
        Player::Empty => format!("[{}]", idx).to_string(),
    }
}
