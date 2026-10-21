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
    /// Apesar do nome ser autoexplicativo, essa função toma para si a struct CLI (note o self e não
    /// &self) e retorna uma configuração do simulador de cache
    pub fn into_config(self) -> Result<SimConfig, String> {
        // Separa as configurações de cache do caminho pro arquivos
        // o caminho para o arquivo deve ser o ultimo argumento passado, por isso o split_last
        let (file, settings) = self.args.split_last().unwrap();
        let file = PathBuf::from(file);

        // Cria um vetor de `Cache` apartir da entrada de args do usuario
        // s.parse::<Cache> vai chamar o metodo from_str que é implementado pelo `Cache`
        let configs = settings
            .iter()
            .map(|s| s.parse::<Cache>())
            .collect::<Result<Vec<_>, String>>()?;

        // para mudar se a a ordem entre config de dados e instruções é aqui
        let (il1, dl1, l2) = match configs.len() {
            2 => (configs[0], configs[1], None),
            3 => (configs[0], configs[1], Some(configs[2])),

            // retorna a configuração padrão caso nenhuma seja passada pelo usuario
            // Temos que definir a configuração padrão ainda né papai
            _ => (Cache::new(0, 0, 0), Cache::new(0, 0, 0), None),
        };

        Ok(SimConfig::new(dl1, il1, l2, file))
    }
}
