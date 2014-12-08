#[cfg(test)]

use tic_tac_toe::io::cli_spy;
use tic_tac_toe::io::IO;

#[test]
fn cli_spy_stores_pritned_values() {
    let mut  spy = cli_spy::new();
    spy.print("sentinel value");
    match spy.last_line() {
        Some(line) => assert_eq!("sentinel value", line.as_slice()),
        None => panic!("No value"),
    }
}

