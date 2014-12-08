pub mod cli;
pub mod cli_spy;
pub mod display;

pub trait IO {
    fn print(&self, line: &str);
    fn read(&self) -> String;
}
