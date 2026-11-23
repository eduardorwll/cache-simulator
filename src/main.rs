use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    process::ExitCode,
};

use clap::Parser;

use crate::{address::Address, args::Cli};

mod address;
mod args;
mod cache;
mod cache_simulator;

/// Lê o próximo endereço de 32 bits do arquivo binário.
/// Retorna `Ok(None)` ao atingir o fim do arquivo em uma fronteira exata de 4 bytes.
fn read_next_addr(reader: &mut BufReader<File>) -> io::Result<Option<u32>> {
    if reader.fill_buf()?.is_empty() {
        return Ok(None);
    }

    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(Some(u32::from_be_bytes(buf)))
}

fn main() -> ExitCode {
    let mut simulator_config = match Cli::parse().into_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("erro: {e}");
            return ExitCode::FAILURE;
        }
    };

    let file = match File::open(&simulator_config.file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "erro ao abrir o arquivo '{}': {e}",
                simulator_config.file.display()
            );
            return ExitCode::FAILURE;
        }
    };

    let mut reader = BufReader::new(file);

    for addr_idx in 0usize.. {
        let addr = match read_next_addr(&mut reader) {
            Ok(Some(addr)) => addr,
            Ok(None) => break,
            Err(e) => {
                eprintln!(
                    "erro ao ler o endereço #{addr_idx} do arquivo '{}': {e}",
                    simulator_config.file.display()
                );
                return ExitCode::FAILURE;
            }
        };

        let address = Address::new(addr);
        simulator_config.access(address);
    }

    simulator_config.print_report();

    ExitCode::SUCCESS
}
