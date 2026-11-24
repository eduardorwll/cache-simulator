use std::collections::HashSet;

use crate::address::{Address, DecodedAddress};

use self::{
    cache_config::{CacheConfig, ReplacementPolicy},
    cache_line::CacheLine,
    fa_lru::FullyAssociativeLru,
    stats::CacheStats,
};
use rand::RngExt;

pub mod cache_config;
pub mod cache_line;
pub mod fa_lru;
pub mod stats;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccessOutcome {
    Hit,
    Miss { replaced: bool },
}

pub struct Cache {
    pub config: CacheConfig,
    pub sets: Vec<Vec<CacheLine>>,
    pub current_time: u64,
    pub stats: CacheStats,
    pub seen_blocks: HashSet<u32>,
    fa_lru: FullyAssociativeLru,
}

impl Cache {
    pub fn new(config: CacheConfig) -> Self {
        let total_blocks = config.nsets * config.assoc;

        Self {
            config,
            sets: vec![vec![CacheLine::default(); config.assoc]; config.nsets],
            current_time: 0,
            stats: CacheStats::default(),
            seen_blocks: HashSet::new(),
            fa_lru: FullyAssociativeLru::new(total_blocks),
        }
    }

    /// Processa um acesso à cache, atualizando os contadores estatísticos,
    /// a cache sombra totalmente associativa LRU (do artigo)
    /// e a linha de cache correspondente conforme a política configurada.
    pub fn access(&mut self, addr: Address) -> AccessOutcome {
        self.current_time = self.current_time.saturating_add(1);
        let now = self.current_time;

        // Alimenta a shadow cache C_FA em TODO acesso (tanto Hit quanto Miss da cache real)
        // para manter sua ordem LRU sincronizada com a sequência real de referências.
        let block_addr = addr
            .addr
            .checked_shr(self.config.n_bits_offset)
            .unwrap_or(0);
        if !self.fa_lru.access(block_addr) {
            self.stats.record_fa_miss();
        }

        let dec_addr = self.decode_address(addr);
        let set_idx = dec_addr.indice as usize;
        let set = &mut self.sets[set_idx];

        // 1. Verifica se já está em cache (Hit)
        if let Some(line) = set.iter_mut().find(|l| l.valid && l.tag == dec_addr.tag) {
            line.last_access = now;
            self.stats.record_hit();
            return AccessOutcome::Hit;
        }

        // 2. Se for Miss, classifica na cache infinita C_inf (compulsório vs não-compulsório)
        let compulsory = self.seen_blocks.insert(block_addr);

        // Procura uma linha vazia (inválida)
        if let Some(empty_line) = set.iter_mut().find(|l| !l.valid) {
            empty_line.valid = true;
            empty_line.tag = dec_addr.tag;
            empty_line.last_access = now;
            empty_line.insert_time = now;
            self.stats.record_miss(false, compulsory);
            return AccessOutcome::Miss { replaced: false };
        }

        // 3. Se todas as vias estão ocupadas, aplica a política de substituição
        let victim_idx = match self.config.policy {
            ReplacementPolicy::Lru => {
                // Linha com o menor last_access
                set.iter()
                    .enumerate()
                    .min_by_key(|(_, line)| line.last_access)
                    .map(|(idx, _)| idx)
                    .unwrap_or(0)
            }
            ReplacementPolicy::Fifo => {
                // Linha com o menor insert_time
                set.iter()
                    .enumerate()
                    .min_by_key(|(_, line)| line.insert_time)
                    .map(|(idx, _)| idx)
                    .unwrap_or(0)
            }
            ReplacementPolicy::Random => {
                let mut rng = rand::rng();
                rng.random_range(0..self.config.assoc)
            }
        };

        let victim = &mut set[victim_idx];
        victim.tag = dec_addr.tag;
        victim.valid = true;
        victim.last_access = now;
        victim.insert_time = now;

        self.stats.record_miss(true, compulsory);
        AccessOutcome::Miss { replaced: true }
    }

    pub fn is_cached(&self, addr: Address) -> bool {
        let dec_addr = self.decode_address(addr);
        let set = &self.sets[dec_addr.indice as usize];

        set.iter().any(|l| l.valid && l.tag == dec_addr.tag)
    }

    pub fn insert(&mut self, addr: Address) {
        self.access(addr);
    }

    pub fn decode_address(&self, addr: Address) -> DecodedAddress {
        let dec = addr.addr;

        let offset = if self.config.n_bits_offset == 0 {
            0
        } else {
            dec & ((1u32 << self.config.n_bits_offset) - 1)
        };

        let indice = if self.config.n_bits_indice == 0 {
            0
        } else {
            (dec >> self.config.n_bits_offset) & ((1u32 << self.config.n_bits_indice) - 1)
        };

        let tag = if self.config.n_bits_offset + self.config.n_bits_indice >= 32 {
            0
        } else {
            dec >> (self.config.n_bits_offset + self.config.n_bits_indice)
        };

        DecodedAddress::new(offset, indice, tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_associative_lru_has_zero_conflict_misses() {
        // Cache real já é totalmente associativa com LRU (3 blocos de 16 B)
        let config = CacheConfig::with_policy(1, 16, 3, ReplacementPolicy::Lru);
        let mut cache = Cache::new(config);

        // Acessos: 0, 1, 2 (3 compulsórios), 0 (hit), 3 (compulsório, expulsa 1),
        // 1 (capacidade, expulsa 2), 2 (capacidade, expulsa 0)
        for addr in [0x00, 0x10, 0x20, 0x00, 0x30, 0x10, 0x20] {
            cache.access(Address::new(addr));
        }

        assert_eq!(cache.stats.misses, 6);
        assert_eq!(cache.stats.compulsory_misses, 4);
        assert_eq!(cache.stats.capacity_misses(), 2);
        assert_eq!(cache.stats.conflict_misses(), 0);
    }

    #[test]
    fn test_direct_mapped_separates_conflict_from_capacity() {
        // Mapeamento direto: 2 conjuntos x 16 B/bloco = capacidade total de 2 blocos.
        // Blocos 0 (0x00) e 2 (0x20) mapeiam ambos para o conjunto 0 (conflito!),
        // enquanto o conjunto 1 fica vazio.
        let config = CacheConfig::with_policy(2, 16, 1, ReplacementPolicy::Lru);
        let mut cache = Cache::new(config);

        for addr in [0x00, 0x20, 0x00, 0x20] {
            cache.access(Address::new(addr));
        }

        // Na cache real: 4 misses (todos no set 0).
        // Na C_FA de 2 blocos: apenas 2 misses (compulsórios) e 2 hits!
        assert_eq!(cache.stats.misses, 4);
        assert_eq!(cache.stats.compulsory_misses, 2);
        assert_eq!(cache.stats.capacity_misses(), 0);
        assert_eq!(cache.stats.conflict_misses(), 2);

        // Agora acessamos um 3º bloco (0x40 -> bloco 4, set 0) e voltamos em 0x00:
        // como só cabem 2 blocos no total, 0x00 agora é miss de CAPACIDADE na C_FA!
        cache.access(Address::new(0x40)); // compulsório (na C_FA expulsa 0x00)
        cache.access(Address::new(0x00)); // capacidade (miss em C_FA e em C)

        assert_eq!(cache.stats.misses, 6);
        assert_eq!(cache.stats.compulsory_misses, 3);
        assert_eq!(cache.stats.capacity_misses(), 1);
        assert_eq!(cache.stats.conflict_misses(), 2);
    }
}
