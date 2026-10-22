#[derive(Copy, Clone, Debug)]
/// Struct da configuração de UMA cache (l1, l2 etc)
pub struct CacheConfig {
    pub nsets: usize,
    pub bsize: usize,
    pub assoc: usize,
    pub n_bits_offset: u32,
    pub n_bits_indice: u32,
    pub n_bits_tag: u32,
}

/// Implementação da trait FromStr para que seja feito o parsing de `<nsets>:<bsize>:<assoc>` para a
/// struct CacheConfig
impl std::str::FromStr for CacheConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid = || {
            format!(
                "config de cache inválida: `{s}` (esperado <nsets>:<bsize>:<assoc>, ex.: 1024:16:4)"
            )
        };

        let parts: Vec<&str> = s.split(":").collect();

        if parts.len() != 3 {
            return Err(invalid());
        }

        let nsets: usize = parts[0].parse().map_err(|_| invalid())?;
        let bsize: usize = parts[1].parse().map_err(|_| invalid())?;
        let assoc: usize = parts[2].parse().map_err(|_| invalid())?;

        Ok(Self::new(nsets, bsize, assoc))
    }
}

/// Implementação de metodos básicos de uma struct
impl CacheConfig {
    pub fn new(nsets: usize, bsize: usize, assoc: usize) -> Self {
        let n_bits_offset = (bsize as f64).log2() as u32;
        let n_bits_indice = (nsets as f64).log2() as u32;
        let n_bits_tag = 32 - n_bits_offset - n_bits_indice;

        Self {
            nsets,
            bsize,
            assoc,
            n_bits_offset,
            n_bits_indice,
            n_bits_tag,
        }
    }
}
