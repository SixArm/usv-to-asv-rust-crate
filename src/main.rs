//! # usv-to-asv
//!
//! Convert [Unicode Separated Values (USV)](https://github.com/sixarm/usv) to ASCII Separated Values (ASV).
//!
//! Syntax:
//!
//! ```sh
//! stdin | usv-to-asv | stdout
//! ```
//!
//! Example:
//!
//! ```sh
//! cat example.usv | usv-to-asv
//! ```
//!
//! Example with output to a file:
//!
//! ```sh
//! cat example.usv | usv-to-asv > example.asv
//! ```
//!
//! ## Options
//!
//! * -h, --help : Print help
//!
//! * -V, --version : Print version
//!
//! * -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …
//!
//! * --test : Print test output for debugging, verifying, tracing, and the like. Example: --test
//!
//!
//! ## Install
//!
//! Install:
//!
//! ```sh
//! cargo install usv-to-asv
//! ```
//!
//! Link: [https://crates.io/crates/usv-to-asv](https://crates.io/crates/usv-to-asv)
//!
//!
//! ## Example
//!
//! Suppose example.usv contains:
//!
//! ```usv
//! a␟b␟␞
//! c␟d␟␞
//! ```
//!
//! Run:
//!
//! ```sh
//! cat example.usv | usv-to-asv
//! ```
//!
//! Output:
//!
//! ```asv
//! a\u001Fb\u001F\u001E
//! c\u001Fd\u001F\u001E
//! ```
//!
//! ## FAQ
//!
//! ### When to use this command?
//!
//! Use this command when you want to convert from USV to ASV.
//!
//! A typical use case is when you have USV data, such as a collection of units and
//! records, and you want to convert it to ASV data, such as for a spreadsheet
//! import.
//!
//! Our real-world use case is converting a bunch of USV document-oriented data
//! from a variety of programs, including a CMS, to USV so we're better-able to
//! import the data into Excel.
//!
//! ### Is there a similar command to convert from ASV to USV?
//!
//! Yes: [asv-to-usv](https://crates.io/crates/asv-to-usv).
//!
//! ### Why use USV instead of ASV?
//!
//! See the documentation for [USV](https://github.com/sixarm/usv).
//!
//! ### Is USV aiming to become a standard?
//!
//! Yes and we've submitted the first draft of the USV standard to the IETF:
//! [link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).
//!
//! ## Help wanted
//!
//! Constructive feedback welcome. Pull requests and feature requests welcome.
//!
//! ## Tracking
//!
//! * Package: usv-to-asv-rust-crate
//! * Version: 1.3.0
//! * Created: 2024-03-09T13:33:20Z
//! * Updated: 2024-03-24T21:52:09Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

//// log
#[macro_use]
extern crate log;
extern crate env_logger;

use usv_to_asv::usv_to_asv;
use std::io::{Read, stdin};

pub mod app {
    pub mod args;
    pub mod clap;
    pub mod log;
}

fn main() -> std::io::Result<()> {
    let args: crate::app::args::Args = crate::app::clap::clap();
    if args.test { println!("{:?}", args); }
    let mut stdin = stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    println!("{}", usv_to_asv(&s));
    Ok(())
}
