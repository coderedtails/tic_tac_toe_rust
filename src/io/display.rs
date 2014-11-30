use line::Player;

pub fn render_line(line: &[Player,..3]) -> String {
    let transformed_line: Vec<String> = line.iter()
                                            .enumerate()
                                            .map(render_cell)
                                            .collect();
    transformed_line.connect("")
}

pub fn render_cell(elements: (uint, &Player)) -> String {
    let (idx, player) = elements;
    match *player {
        Player::X => "[X]".to_string(),
        Player::O => "[O]".to_string(),
        Player::Empty => format!("[{}]", idx).to_string(),
    }
}
