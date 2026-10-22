use std::process::ExitCode;

use clap::Parser;

use crate::args::Cli;

mod address;
mod args;
mod cache;
mod cache_simulator;

fn main() -> ExitCode {
    let simulator_config = match Cli::parse().into_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("erro: {e}");
            return ExitCode::FAILURE;
        }
    };

    // for nos endereços do arquivo de entrada
    let addr: u32 = 0;

    // end for

    ExitCode::SUCCESS
}
