use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ReplacementPolicy {
    #[default]
    Lru,
    Fifo,
    Random,
}

impl FromStr for ReplacementPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "l" | "lru" => Ok(ReplacementPolicy::Lru),
            "f" | "fifo" => Ok(ReplacementPolicy::Fifo),
            "r" | "random" => Ok(ReplacementPolicy::Random),
            other => Err(format!(
                "política de substituição inválida: `{other}` (esperado 'l' (LRU), 'f' (FIFO) ou 'r' (RANDOM))"
            )),
        }
    }
}

impl std::fmt::Display for ReplacementPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplacementPolicy::Lru => write!(f, "LRU (l)"),
            ReplacementPolicy::Fifo => write!(f, "FIFO (f)"),
            ReplacementPolicy::Random => write!(f, "RANDOM (r)"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Struct da configuração de UMA cache (l1, l2 etc)
pub struct CacheConfig {
    pub nsets: usize,
    pub bsize: usize,
    pub assoc: usize,
    pub policy: ReplacementPolicy,
    pub n_bits_offset: u32,
    pub n_bits_indice: u32,
    pub n_bits_tag: u32,
}

/// Implementação da trait FromStr para que seja feito o parsing de
/// `<nsets>:<bsize>:<assoc>` ou `<nsets>:<bsize>:<assoc>:<repl>`
impl FromStr for CacheConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid = || {
            format!(
                "config de cache inválida: `{s}` (esperado <nsets>:<bsize>:<assoc>[:<repl>], ex.: 1024:16:4 ou 1024:16:4:l)"
            )
        };

        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 3 && parts.len() != 4 {
            return Err(invalid());
        }

        let nsets: usize = parts[0].parse().map_err(|_| invalid())?;
        let bsize: usize = parts[1].parse().map_err(|_| invalid())?;
        let assoc: usize = parts[2].parse().map_err(|_| invalid())?;

        let policy = if parts.len() == 4 {
            parts[3].parse::<ReplacementPolicy>()?
        } else {
            ReplacementPolicy::default()
        };

        Self::build(nsets, bsize, assoc, policy)
    }
}

impl CacheConfig {
    pub fn new(nsets: usize, bsize: usize, assoc: usize) -> Self {
        Self::with_policy(nsets, bsize, assoc, ReplacementPolicy::default())
    }

    pub fn with_policy(
        nsets: usize,
        bsize: usize,
        assoc: usize,
        policy: ReplacementPolicy,
    ) -> Self {
        Self::build(nsets, bsize, assoc, policy)
            .expect("Parâmetros de configuração de cache inválidos")
    }

    pub fn build(
        nsets: usize,
        bsize: usize,
        assoc: usize,
        policy: ReplacementPolicy,
    ) -> Result<Self, String> {
        if bsize == 0 || !bsize.is_power_of_two() {
            return Err(format!(
                "tamanho do bloco ({bsize}) deve ser uma potência de 2 maior que 0"
            ));
        }

        if nsets == 0 || !nsets.is_power_of_two() {
            return Err(format!(
                "número de conjuntos ({nsets}) deve ser uma potência de 2 maior que 0"
            ));
        }

        if assoc == 0 {
            return Err("associatividade deve ser maior que 0".to_string());
        }

        let n_bits_offset = bsize.trailing_zeros();
        let n_bits_indice = nsets.trailing_zeros();

        if n_bits_offset + n_bits_indice > 32 {
            return Err(format!(
                "soma dos bits de offset ({n_bits_offset}) e índice ({n_bits_indice}) excede 32 bits"
            ));
        }

        let n_bits_tag = 32 - n_bits_offset - n_bits_indice;

        Ok(Self {
            nsets,
            bsize,
            assoc,
            policy,
            n_bits_offset,
            n_bits_indice,
            n_bits_tag,
        })
    }
}

