use std::path::PathBuf;

use crate::cache::Cache;

/// Struct das configurações do simulador de cache
pub struct SimConfig {
    pub dl1: Cache,
    pub il1: Cache,
    pub l2: Option<Cache>,
    pub file: PathBuf,
}

/// Implementação de metodos básicos de uma struct
impl SimConfig {
    pub fn new(dl1: Cache, il1: Cache, l2: Option<Cache>, file: PathBuf) -> Self {
        Self { dl1, il1, l2, file }
    }
}
