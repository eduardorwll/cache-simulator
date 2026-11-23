use std::path::PathBuf;

use crate::cache::Cache;

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

    /// Exibe um relatório consolidado de todas as caches simuladas,
    /// no padrão adotado nos relatórios de experimentos da disciplina AOC II.
    pub fn print_report(&self) {
        println!("============================================================");
        println!("               RELATÓRIO DO SIMULADOR DE CACHE              ");
        println!("============================================================");
        println!("Arquivo analisado: {}", self.file.display());
        println!("------------------------------------------------------------");

        // IL1
        println!("[Cache L1 - Instruções]");
        println!(
            "  Configuração: {} conjuntos, {} B/bloco, {}-way, política {}",
            self.il1.config.nsets,
            self.il1.config.bsize,
            self.il1.config.assoc,
            self.il1.config.policy
        );
        println!(
            "  Instruções executadas (Acessos): {}",
            self.il1.stats.accesses
        );
        println!("  Hits:                            {}", self.il1.stats.hits);
        println!(
            "  Misses:                          {}",
            self.il1.stats.misses
        );
        println!(
            "  Taxa de Miss:                    {:.2}%",
            self.il1.stats.miss_rate()
        );
        println!(
            "  Substituições (Replacements):    {}",
            self.il1.stats.replacements
        );
        println!("------------------------------------------------------------");

        // DL1
        println!("[Cache L1 - Dados]");
        println!(
            "  Configuração: {} conjuntos, {} B/bloco, {}-way, política {}",
            self.dl1.config.nsets,
            self.dl1.config.bsize,
            self.dl1.config.assoc,
            self.dl1.config.policy
        );
        println!(
            "  Instruções L/S (Acessos):        {}",
            self.dl1.stats.accesses
        );
        println!("  Hits:                            {}", self.dl1.stats.hits);
        println!(
            "  Misses:                          {}",
            self.dl1.stats.misses
        );
        println!(
            "  Taxa de Miss:                    {:.2}%",
            self.dl1.stats.miss_rate()
        );
        println!(
            "  Substituições (Replacements):    {}",
            self.dl1.stats.replacements
        );
        println!("------------------------------------------------------------");

        // L2 (se presente)
        if let Some(l2) = &self.l2 {
            println!("[Cache L2 - Unificada]");
            println!(
                "  Configuração: {} conjuntos, {} B/bloco, {}-way, política {}",
                l2.config.nsets, l2.config.bsize, l2.config.assoc, l2.config.policy
            );
            println!("  Acessos:                         {}", l2.stats.accesses);
            println!("  Hits:                            {}", l2.stats.hits);
            println!("  Misses:                          {}", l2.stats.misses);
            println!(
                "  Taxa de Miss:                    {:.2}%",
                l2.stats.miss_rate()
            );
            println!(
                "  Substituições (Replacements):    {}",
                l2.stats.replacements
            );
            println!("------------------------------------------------------------");
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
