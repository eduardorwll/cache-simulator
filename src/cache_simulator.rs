use std::path::PathBuf;

use crate::{
    address::{AddrKind, Address},
    cache::{AccessOutcome, Cache},
};

/// Struct de gerenciamento do simulador de cache
pub struct SimConfig {
    pub dl1: Cache,
    pub il1: Cache,
    pub l2: Option<Cache>,
    pub file: PathBuf,
}

impl SimConfig {
    pub fn new(dl1: Cache, il1: Cache, l2: Option<Cache>, file: PathBuf) -> Self {
        Self { dl1, il1, l2, file }
    }

    pub fn access(&mut self, addr: Address) {
        let l1_outcome = match addr.kind {
            AddrKind::Instruction => self.il1.access(addr),
            AddrKind::Data => self.dl1.access(addr),
        };

        if let AccessOutcome::Miss { .. } = l1_outcome {
            if let Some(l2) = &mut self.l2 {
                l2.access(addr);
            }
        }
    }

    fn format_size(bytes: usize) -> String {
        const UNITS: [(usize, &str); 3] = [
            (1024 * 1024 * 1024, "GiB"),
            (1024 * 1024, "MiB"),
            (1024, "KiB"),
        ];

        for (unit_size, label) in UNITS {
            if bytes >= unit_size {
                return if bytes.is_multiple_of(unit_size) {
                    format!("{} {label} ({bytes} B)", bytes / unit_size)
                } else {
                    format!("{:.2} {label} ({bytes} B)", bytes as f64 / unit_size as f64)
                };
            }
        }

        format!("{bytes} B")
    }

    fn print_cache_section(title: &str, access_label: &str, cache: &Cache) {
        println!("[{title}]");
        println!(
            "  Configuração: {} conjuntos, {} B/bloco, {}-way, política {}",
            cache.config.nsets, cache.config.bsize, cache.config.assoc, cache.config.policy
        );
        println!(
            "  Tamanho total:                   {}",
            Self::format_size(cache.config.total_size_bytes())
        );
        println!("  {access_label:<32} {}", cache.stats.accesses);
        println!("  Hits:                            {}", cache.stats.hits);
        println!(
            "  Taxa de Hit (Hit Ratio):         {:.2}%",
            cache.stats.hit_rate()
        );
        println!("  Misses:                          {}", cache.stats.misses);
        println!(
            "    - Compulsórios:                {} ({:.2}% dos misses)",
            cache.stats.compulsory_misses,
            cache.stats.compulsory_miss_ratio()
        );
        println!(
            "    - Capacidade + Conflito:       {} ({:.2}% dos misses)",
            cache.stats.capacity_conflict_misses,
            cache.stats.capacity_conflict_miss_ratio()
        );
        println!(
            "  Taxa de Miss (Miss Ratio):       {:.2}%",
            cache.stats.miss_rate()
        );
        println!(
            "  Substituições (Replacements):    {}",
            cache.stats.replacements
        );
        println!("------------------------------------------------------------");
    }

    /// Exibe um relatório consolidado de todas as caches simuladas.
    pub fn print_report(&self) {
        println!("============================================================");
        println!("               RELATÓRIO DO SIMULADOR DE CACHE              ");
        println!("============================================================");
        println!("Arquivo analisado: {}", self.file.display());
        println!("------------------------------------------------------------");

        Self::print_cache_section(
            "Cache L1 - Instruções",
            "Instruções executadas (Acessos):",
            &self.il1,
        );
        Self::print_cache_section("Cache L1 - Dados", "Instruções L/S (Acessos):", &self.dl1);

        if let Some(l2) = &self.l2 {
            Self::print_cache_section("Cache L2 - Unificada", "Acessos:", l2);
        }

        let total_accesses = self.il1.stats.accesses + self.dl1.stats.accesses;
        let total_misses = self.il1.stats.misses + self.dl1.stats.misses;
        let global_miss_rate = if total_accesses > 0 {
            (total_misses as f64 / total_accesses as f64) * 100.0
        } else {
            0.0
        };

        println!("[Estatísticas Globais L1]");
        println!("  Total de Acessos L1:             {}", total_accesses);
        println!("  Total de Misses L1:              {}", total_misses);
        println!(
            "  Taxa Global de Miss L1:          {:.2}%",
            global_miss_rate
        );
        println!("============================================================");
    }
}
