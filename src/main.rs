use std::process::ExitCode;

use clap::Parser;

use crate::args::Cli;

mod args;
mod cache;

fn main() -> ExitCode {
    let _config = match Cli::parse().into_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("erro: {e}");
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
