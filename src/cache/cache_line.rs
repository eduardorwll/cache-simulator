#[derive(Clone, Default)]
pub struct CacheLine {
    pub tag: u32,
    pub valid: bool,
}

impl CacheLine {
    pub fn new(tag: u32, valid: bool) -> Self {
        Self { tag, valid }
    }
}
