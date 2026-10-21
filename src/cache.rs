use core::str;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug)]
pub struct Cache {
    nsets: usize,
    bsize: usize,
    assoc: usize,
}

pub struct SimConfig {
    dl1: Cache,
    il1: Cache,
    l2: Option<Cache>,
    file: PathBuf,
}

impl SimConfig {
    pub fn new(dl1: Cache, il1: Cache, l2: Option<Cache>, file: PathBuf) -> Self {
        Self { dl1, il1, l2, file }
    }

    pub fn dl1(&self) -> Cache {
        self.dl1
    }
    pub fn il1(&self) -> Cache {
        self.il1
    }
    pub fn l2(&self) -> Option<Cache> {
        self.l2
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }
}

impl std::str::FromStr for Cache {
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

        Ok(Self {
            nsets: parts[0].parse().map_err(|_| invalid())?,
            bsize: parts[1].parse().map_err(|_| invalid())?,
            assoc: parts[2].parse().map_err(|_| invalid())?,
        })
    }
}

impl Cache {
    pub fn new(nsets: usize, bsize: usize, assoc: usize) -> Self {
        Self {
            nsets,
            bsize,
            assoc,
        }
    }

    pub fn nsets(&self) -> usize {
        self.nsets
    }

    pub fn bsize(&self) -> usize {
        self.bsize
    }

    pub fn assoc(&self) -> usize {
        self.assoc
    }
}
