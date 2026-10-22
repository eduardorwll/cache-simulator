use crate::address::{Address, DecodedAddress};

use self::{cache_config::CacheConfig, cache_line::CacheLine};
use rand::RngExt;

pub mod cache_config;
mod cache_line;

pub struct Cache {
    pub config: CacheConfig,
    pub sets: Vec<Vec<CacheLine>>,
}

impl Cache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            // vec![x; n] => um vetor com n elementos x. Ex: vec![0; 5] => [0, 0, 0, 0, 0]
            sets: vec![vec![CacheLine::default(); config.assoc]; config.nsets],
        }
    }

    pub fn is_cached(&self, addr: Address) -> bool {
        let dec_addr = self.decode_address(addr);
        let set = &self.sets[dec_addr.indice as usize];

        set.iter().any(|l| l.valid && l.tag == dec_addr.tag)
    }

    pub fn insert(&mut self, addr: Address) {
        if self.is_cached(addr) {
            return;
        }

        let dec_addr = self.decode_address(addr);

        let mut rng = rand::rng();
        let i: usize = rng.random_range(0..self.config.assoc);

        self.sets[dec_addr.indice as usize][i].tag = dec_addr.tag;
        self.sets[dec_addr.indice as usize][i].valid = true;
    }

    pub fn decode_address(&self, addr: Address) -> DecodedAddress {
        let dec = addr.addr;
        let offset = dec & (2u32.pow(self.config.n_bits_offset) - 1);
        let indice = dec >> self.config.n_bits_offset & (2u32.pow(self.config.n_bits_indice) - 1);
        let tag = dec >> (self.config.n_bits_offset + self.config.n_bits_indice);

        DecodedAddress::new(offset, indice, tag)
    }
}
