use clap::Parser;

use crate::args::Cli;

mod args;
mod cache;

fn main() {
    let _config = Cli::parse().into_config();
}
