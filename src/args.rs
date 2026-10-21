use crate::cache::{Cache, SimConfig};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_name = "ARGS", num_args = 1..=4)]
    args: Vec<String>,
}

impl Cli {
    pub fn into_config(self) -> Result<SimConfig, String> {
        let (file, settings) = self.args.split_last().unwrap();
        let file = PathBuf::from(file);

        let configs = settings
            .iter()
            .map(|s| s.parse::<Cache>())
            .collect::<Result<Vec<_>, String>>()?;

        let (il1, dl1, l2) = match configs.len() {
            2 => (configs[0], configs[1], None),
            3 => (configs[0], configs[1], Some(configs[2])),

            _ => (Cache::new(0, 0, 0), Cache::new(0, 0, 0), None),
        };

        Ok(SimConfig::new(dl1, il1, l2, file))
    }
}
