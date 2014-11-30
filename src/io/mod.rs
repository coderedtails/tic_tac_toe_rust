pub mod display;
pub mod cli;
pub mod cli_spy;

pub trait Printer {
    fn print(&self, line: String);
}

pub trait Reader {
    fn read_line(&self) -> uint;
}
