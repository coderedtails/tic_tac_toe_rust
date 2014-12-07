use players::Player;
use std::cell::Cell;
use io::display::Display;
use io::cli::Cli;
use core::marker::Marker;
use players::ai;
use players::human;

pub struct GameMode<'a>{
    pub first:  Box<Player + 'a>,
    pub second: Box<Player + 'a>,
    pub counter: Cell<uint>,
}

impl<'a> GameMode<'a> {
   pub fn next_player(&self) -> &Player {
       let val: uint = self.counter.get();
       let player: &Player = if val % 2 == 0 {
           &*self.first
       } else {
           &*self.second
        };

        self.counter.set(val + 1);
        player
   }
}

impl<'a> GameMode<'a> {
    pub fn opponents_line(&self) -> String {
        format!("{} vs. {}", self.first.player_type(), self.second.player_type())
    }
}

pub fn new<'a>(first: Box<Player + 'a>, second: Box<Player + 'a>) -> GameMode<'a> {
    GameMode { first: first, second: second, counter: Cell::new(0u) }
}

pub fn choose_game_mode<'a>(display: Display<Cli>) -> GameMode<'a> {
    let possible_modes = create_game_modes(display);
    loop {
        display.show_options(&possible_modes);
        let choice = display.request_mode();
        if choice <= possible_modes.len() {
           return possible_modes[choice]
        }
    }
}

pub fn create_game_modes<'a>(display: Display<Cli>) -> [GameMode<'a>, ..4] {
    [human_vs_ai(display),
     human_vs_human(display),
     ai_vs_human(display),
     ai_vs_ai()]
}

fn human_vs_ai<'a>(display: Display<Cli>) -> GameMode<'a> {
    let human = human::new(Marker::X, display);
    let ai = ai::new(Marker::O);
    new(box human, box ai)
}

fn ai_vs_human<'a>(display: Display<Cli>) -> GameMode<'a> {
    let ai = ai::new(Marker::O);
    let human = human::new(Marker::O, display);
    new(box ai, box human)
}

fn human_vs_human<'a>(display: Display<Cli>) -> GameMode<'a> {
    let human1 = human::new(Marker::X, display);
    let human2 = human::new(Marker::O, display);
    new(box human1, box human2)
}

pub fn ai_vs_ai<'a>() -> GameMode<'a> {
    let ai1 = ai::new(Marker::X);
    let ai2 = ai::new(Marker::O);
    new(box ai1, box ai2)
}
