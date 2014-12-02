pub mod display;
pub mod cli;
pub mod cli_spy;

pub trait IO {
    fn print(&self, line: String);
    fn read(&self) -> String;
}
