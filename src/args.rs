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
Configuração de cache:
  <nsets>:<bsize>:<assoc>[:<repl>]
  - nsets: número de conjuntos (potência de 2, ex: 1024, ou 1 para totalmente associativa)
  - bsize: tamanho do bloco em bytes (potência de 2, ex: 16, 32)
  - assoc: associatividade / vias por conjunto (ex: 1, 2, 4, 16)
  - repl: política de substituição (opcional, padrão: l)
      l: LRU (Least Recently Used)
      f: FIFO (First In First Out)
      r: RANDOM

Exemplos:
  cache-simulator 256:16:4 256:16:4 benchmark.bin
  cache-simulator 1:32:16:l 1:32:16:l benchmark.bin
  cache-simulator 1:32:16:f 1:32:16:f benchmark.bin
  cache-simulator 256:16:4:l 256:16:4:l 4096:32:8:l benchmark.bin")]
pub struct Cli {
    #[arg(value_name = "ARGS", num_args = 1..=4)]
    args: Vec<String>,
}

impl Cli {
    /// Apesar do nome ser autoexplicativo, essa função toma para si a struct CLI (note o self e não
    /// &self) e retorna uma configuração do simulador de cache
    pub fn into_config(self) -> Result<SimConfig, String> {
        // Separa as configurações de cache do caminho pro arquivo
        // o caminho para o arquivo deve ser o ultimo argumento passado, por isso o split_last
        let (file, settings) = self.args.split_last().unwrap();
        let file = PathBuf::from(file);

        if !file.exists() {
            return Err(format!(
                "O arquivo {} não existe! Tente passar um arquivo que existe e no formato válido",
                file.display()
            ));
        }

        if !file.is_file() {
            return Err(format!(
                "O caminho {} não é um arquivo válido",
                file.display()
            ));
        }

        // cria um vetor de CacheConfig a partir da entrada de args do usuário
        let configs = settings
            .iter()
            .map(|s| s.parse::<CacheConfig>())
            .collect::<Result<Vec<_>, String>>()?;

        // para mudar a ordem entre config de dados e instruções é aqui
        let (il1, dl1, l2) = match configs.len() {
            // retorna a configuração padrão caso nenhuma seja passada pelo usuario
            0 => {
                let default_il1 = CacheConfig::new(64, 4, 1);
                let default_dl1 = CacheConfig::new(64, 4, 1);
                let default_l2 = CacheConfig::new(64, 4, 1);

                (
                    Cache::new(default_il1),
                    Cache::new(default_dl1),
                    Some(Cache::new(default_l2)),
                )
            }
            2 => (Cache::new(configs[0]), Cache::new(configs[1]), None),
            3 => (
                Cache::new(configs[0]),
                Cache::new(configs[1]),
                Some(Cache::new(configs[2])),
            ),
            _ => {
                return Err(
                    "Configuração inválida. Esperado 2 ou 3 configurações de cache antes do arquivo de benchmark. Use --help para obter mais informações".to_string(),
                );
            }
        };

        Ok(SimConfig::new(dl1, il1, l2, file))
    }
}
