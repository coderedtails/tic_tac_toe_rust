use line::Player;

pub fn render_slice(line: &[Player]) -> Vec<String> {
     line.iter().enumerate().map(render_cell).collect()
}

pub fn render_line(line: &[Player]) -> String {
    render_slice(line).concat()
}

pub fn render_cell(elements: (uint, &Player)) -> String {
    let (idx, player) = elements;
    match *player {
        Player::X => "[X]".to_string(),
        Player::O => "[O]".to_string(),
        Player::Empty => format!("[{}]", idx).to_string(),
    }
}
