use std::fmt;

/// Estatísticas de desempenho da cache, incluindo classificação 3Cs
/// segundo o artigo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub accesses: u64,
    pub hits: u64,
    pub misses: u64,
    pub compulsory_misses: u64,
    pub capacity_conflict_misses: u64,
    /// Total de misses da cache sombra totalmente associativa LRU de mesmo tamanho (C_FA).
    pub fa_misses: u64,
    pub replacements: u64,
}

impl CacheStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra uma ocorrência de hit.
    pub fn record_hit(&mut self) {
        self.accesses = self.accesses.saturating_add(1);
        self.hits = self.hits.saturating_add(1);
    }

    /// Registra um miss na cache real.
    /// `replaced` indica se houve substituição de uma linha que antes era válida.
    /// `compulsory` indica se o bloco nunca havia sido acessado antes nesta cache (C_inf).
    pub fn record_miss(&mut self, replaced: bool, compulsory: bool) {
        self.accesses = self.accesses.saturating_add(1);
        self.misses = self.misses.saturating_add(1);
        if compulsory {
            self.compulsory_misses = self.compulsory_misses.saturating_add(1);
        } else {
            self.capacity_conflict_misses = self.capacity_conflict_misses.saturating_add(1);
        }
        if replaced {
            self.replacements = self.replacements.saturating_add(1);
        }
    }

    /// Registra um miss ocorrido na cache sombra totalmente associativa LRU (C_FA).
    pub fn record_fa_miss(&mut self) {
        self.fa_misses = self.fa_misses.saturating_add(1);
    }

    /// Misses de capacidade segundo o artigo:
    /// Capacity = Misses(C_FA) - Misses(C_inf).
    pub fn capacity_misses(&self) -> u64 {
        self.fa_misses.saturating_sub(self.compulsory_misses)
    }

    /// Misses de conflito segundo o artigo:
    /// Conflict = Misses(C) - Misses(C_FA).
    ///
    /// Retorna `i64` pois, como observado no próprio artigo (p. 1624, Tabela III),
    /// uma cache associativa por conjunto (ou com política diferente de LRU) pode
    /// raramente apresentar ligeiramente menos misses do que C_FA, resultando em
    /// um pequeno valor negativo que preserva a identidade:
    /// `compulsory_misses + capacity_misses + conflict_misses == misses`.
    pub fn conflict_misses(&self) -> i64 {
        self.misses as i64 - self.fa_misses as i64
    }

    /// Taxa de acertos em porcentagem.
    pub fn hit_rate(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            (self.hits as f64 / self.accesses as f64) * 100.0
        }
    }

    /// Taxa de falhas em porcentagem.
    pub fn miss_rate(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            (self.misses as f64 / self.accesses as f64) * 100.0
        }
    }

    /// Percentual de misses compulsórios em relação ao total de misses.
    pub fn compulsory_miss_ratio(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            (self.compulsory_misses as f64 / self.misses as f64) * 100.0
        }
    }

    /// Percentual de misses de capacidade + conflito em relação ao total de misses.
    pub fn capacity_conflict_miss_ratio(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            (self.capacity_conflict_misses as f64 / self.misses as f64) * 100.0
        }
    }

    /// Percentual de misses de capacidade em relação ao total de misses.
    pub fn capacity_miss_ratio(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            (self.capacity_misses() as f64 / self.misses as f64) * 100.0
        }
    }

    /// Percentual de misses de conflito em relação ao total de misses.
    pub fn conflict_miss_ratio(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            (self.conflict_misses() as f64 / self.misses as f64) * 100.0
        }
    }

    /// Reinicia todos os contadores para zero.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Display for CacheStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Accesses: {} | Hits: {} ({:.2}%) | Misses: {} ({:.2}%) | Replacements: {}",
            self.accesses,
            self.hits,
            self.hit_rate(),
            self.misses,
            self.miss_rate(),
            self.replacements
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_cs_decomposition() {
        let mut stats = CacheStats::new();

        // 10 misses no total: 4 compulsórios, 6 não-compulsórios
        for _ in 0..4 {
            stats.record_miss(false, true);
            stats.record_fa_miss(); // todo compulsório também é miss em C_FA
        }
        // Mais 3 misses que também ocorrem na C_FA -> capacidade
        for _ in 0..3 {
            stats.record_miss(true, false);
            stats.record_fa_miss();
        }
        // Mais 3 misses que só ocorrem na cache real -> conflito
        for _ in 0..3 {
            stats.record_miss(true, false);
        }

        assert_eq!(stats.misses, 10);
        assert_eq!(stats.compulsory_misses, 4);
        assert_eq!(stats.fa_misses, 7);
        assert_eq!(stats.capacity_misses(), 3);
        assert_eq!(stats.conflict_misses(), 3);

        // Identidade exata dos 3Cs
        assert_eq!(
            stats.compulsory_misses as i64
                + stats.capacity_misses() as i64
                + stats.conflict_misses(),
            stats.misses as i64
        );
        assert_eq!(
            stats.capacity_misses() as i64 + stats.conflict_misses(),
            stats.capacity_conflict_misses as i64
        );

        assert!((stats.compulsory_miss_ratio() - 40.0).abs() < 1e-6);
        assert!((stats.capacity_miss_ratio() - 30.0).abs() < 1e-6);
        assert!((stats.conflict_miss_ratio() - 30.0).abs() < 1e-6);
    }

    #[test]
    fn test_negative_conflict_miss_preserves_identity() {
        let mut stats = CacheStats::new();

        // 2 compulsórios
        for _ in 0..2 {
            stats.record_miss(false, true);
            stats.record_fa_miss();
        }
        // C_FA teve 2 misses extras (capacidade = 2), mas a cache real só teve 1
        stats.record_miss(true, false);
        stats.record_fa_miss();
        stats.record_fa_miss();

        assert_eq!(stats.misses, 3);
        assert_eq!(stats.compulsory_misses, 2);
        assert_eq!(stats.capacity_misses(), 2);
        assert_eq!(stats.conflict_misses(), -1);

        // Mesmo com conflito negativo, a soma dos 3Cs continua exata
        assert_eq!(
            stats.compulsory_misses as i64
                + stats.capacity_misses() as i64
                + stats.conflict_misses(),
            stats.misses as i64
        );
        let ratio_sum = stats.compulsory_miss_ratio()
            + stats.capacity_miss_ratio()
            + stats.conflict_miss_ratio();
        assert!((ratio_sum - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_zero_misses_ratios_are_zero() {
        let stats = CacheStats::new();
        assert_eq!(stats.capacity_misses(), 0);
        assert_eq!(stats.conflict_misses(), 0);
        assert_eq!(stats.compulsory_miss_ratio(), 0.0);
        assert_eq!(stats.capacity_miss_ratio(), 0.0);
        assert_eq!(stats.conflict_miss_ratio(), 0.0);
    }
}
