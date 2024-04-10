mod common; use common::*;
use std::process::Command;

pub const EXAMPLE_ASV_FILES: &str = "a\u{001F}b\u{001E}c\u{001F}d\u{001D}e\u{001F}f\u{001E}g\u{001F}h\u{001C}i\u{001F}j\u{001E}k\u{001F}l\u{001D}m\u{001F}n\u{001E}o\u{001F}p\u{001C}";
pub const EXAMPLE_USV_FILES: &str = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command, EXAMPLE_USV_FILES);
    assert_eq!(actual, format!("{}\n", EXAMPLE_ASV_FILES));
}
