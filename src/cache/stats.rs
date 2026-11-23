use std::fmt;

/// stats de desempenho da cache

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub accesses: u64,
    pub hits: u64,
    pub misses: u64,
    pub compulsory_misses: u64,
    pub capacity_conflict_misses: u64,
    pub replacements: u64,
}

impl CacheStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// registra uma ocorrência um hit
    pub fn record_hit(&mut self) {
        self.accesses = self.accesses.saturating_add(1);
        self.hits = self.hits.saturating_add(1);
    }

    /// registra um miss
    /// replaced indica se houve substituição de uma linha que antes era valida
    /// compulsory indica se o bloco nunca havia sido acessado antes nesta cache
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

    /// taxa de acertos em porcentagem
    pub fn hit_rate(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            (self.hits as f64 / self.accesses as f64) * 100.0
        }
    }

    /// taxa de falhas em porcentagem
    pub fn miss_rate(&self) -> f64 {
        if self.accesses == 0 {
            0.0
        } else {
            (self.misses as f64 / self.accesses as f64) * 100.0
        }
    }

    /// percentual de misses compulsórios em relação ao total de misses
    pub fn compulsory_miss_ratio(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            (self.compulsory_misses as f64 / self.misses as f64) * 100.0
        }
    }

    /// percentual de misses de capacidade + conflito em relação ao total de misses
    pub fn capacity_conflict_miss_ratio(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            (self.capacity_conflict_misses as f64 / self.misses as f64) * 100.0
        }
    }

    /// reinicia todos os contadores para zero
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
