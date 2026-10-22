use crate::{
    cache::{Cache, cache_config::CacheConfig},
    cache_simulator::SimConfig,
};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
#[command(
    override_usage = "cache-simulator <CONFIG_L1_INSTRUÇÕES> <CONFIG_L1_DADOS> [CONFIG_L2] <ARQUIVO>"
)]
#[command(after_help = "\
Exemplos:
  cache-simulator 256:16:4 256:16:4 benchmark.bin
  cache-simulator 256:16:4 256:16:4 4096:32:8 benchmark.bin")]
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

        if !file.exists() {
            return Err(format!(
                "O arquivo {} não existe! Tente passar um arquivo que existe e no formato válido",
                file.display()
            ));
        }

        // Cria um vetor de `Cache` apartir da entrada de args do usuario
        // s.parse::<CacheConfig> vai chamar o metodo from_str que é implementado pelo `Cache`
        let configs = settings
            .iter()
            .map(|s| s.parse::<CacheConfig>())
            .collect::<Result<Vec<_>, String>>()?;

        // para mudar a ordem entre config de dados e instruções é aqui
        let (il1, dl1, l2) = match configs.len() {
            // retorna a configuração padrão caso nenhuma seja passada pelo usuario
            0 => {
                let default_il1 = CacheConfig::new(256, 16, 4);
                let default_dl1 = CacheConfig::new(256, 16, 4);

                (Cache::new(default_il1), Cache::new(default_dl1), None)
            }
            2 => (Cache::new(configs[0]), Cache::new(configs[1]), None),
            3 => (
                Cache::new(configs[0]),
                Cache::new(configs[1]),
                Some(Cache::new(configs[2])),
            ),
            _ => {
                return Err(
                    "Configuração invalida, use --help para obter mais informações".to_string(),
                );
            }
        };

        Ok(SimConfig::new(dl1, il1, l2, file))
    }
}
