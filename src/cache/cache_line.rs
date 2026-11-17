#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CacheLine {
    pub tag: u32,
    pub valid: bool,
    pub last_access: u64,
    pub insert_time: u64,
}

impl CacheLine {
    pub fn new(tag: u32, valid: bool, last_access: u64, insert_time: u64) -> Self {
        Self {
            tag,
            valid,
            last_access,
            insert_time,
        }
    }
}
