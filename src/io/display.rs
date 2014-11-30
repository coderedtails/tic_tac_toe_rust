use line::Player;

pub fn render_line(line: &[Player]) -> String {
    line.iter().enumerate().map(render_cell).collect::<Vec<String>>().concat()
}

pub fn render_cell(elements: (uint, &Player)) -> String {
    let (idx, player) = elements;
    match *player {
        Player::X => "[X]".to_string(),
        Player::O => "[O]".to_string(),
        Player::Empty => format!("[{}]", idx).to_string(),
    }
}
