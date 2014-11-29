use line::Player;

pub fn render(line: &[Player,..3]) -> String {
    let transformed_line: Vec<String> = line.iter().enumerate().map(render_tuple).collect();
    transformed_line.connect("")
}


fn render_tuple(elements: (uint, &Player)) -> String {
    let (idx, player) = elements;
    render_cell(player, idx)
}

pub fn render_cell(player: &Player, idx: uint) -> String {
    match *player {
        Player::X => "[X]".to_string(),
        Player::O => "[O]".to_string(),
        Player::Empty => format!("[{}]", idx).to_string(),
    }
}
