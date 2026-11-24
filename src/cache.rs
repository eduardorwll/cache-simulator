use std::collections::HashSet;

use crate::address::{Address, DecodedAddress};

use self::{
    cache_config::{CacheConfig, ReplacementPolicy},
    cache_line::CacheLine,
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
}

impl Cache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            sets: vec![vec![CacheLine::default(); config.assoc]; config.nsets],
            current_time: 0,
            stats: CacheStats::default(),
            seen_blocks: HashSet::new(),
        }
    }

    /// Processa um acesso à cache, atualizando os contadores estatísticos
    /// e a linha de cache correspondente conforme a política configurada.
    pub fn access(&mut self, addr: Address) -> AccessOutcome {
        self.current_time = self.current_time.saturating_add(1);
        let now = self.current_time;

        let dec_addr = self.decode_address(addr);
        let set_idx = dec_addr.indice as usize;
        let set = &mut self.sets[set_idx];

        // 1. Verifica se já está em cache (Hit)
        if let Some(line) = set.iter_mut().find(|l| l.valid && l.tag == dec_addr.tag) {
            line.last_access = now;
            self.stats.record_hit();
            return AccessOutcome::Hit;
        }

        // 2. Se for Miss, classifica (compulsório vs capacidade+conflito)
        let block_addr = addr.addr >> self.config.n_bits_offset;
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
