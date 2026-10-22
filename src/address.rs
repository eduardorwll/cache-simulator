pub struct DecodedAddress {
    pub offset: u32,
    pub indice: u32,
    pub tag: u32,
}

impl DecodedAddress {
    pub fn new(offset: u32, indice: u32, tag: u32) -> Self {
        Self {
            offset,
            indice,
            tag,
        }
    }
}

#[derive(Copy, Debug, Clone)]
pub enum AddrKind {
    Data,
    Instruction,
}

#[derive(Copy, Debug, Clone)]
pub struct Address {
    pub addr: u32,
    pub kind: AddrKind,
}

impl Address {
    pub fn new(addr: u32) -> Self {
        Self {
            addr,
            kind: get_addr_kinda(addr),
        }
    }
}

fn get_addr_kinda(addr: u32) -> AddrKind {
    // confirmar se o professor queria dizer MegaBytes ou MibiBytes
    // 2^29 = 512MiB
    if addr < 2u32.pow(29) {
        AddrKind::Data
    } else {
        AddrKind::Instruction
    }
}

// analisar a necessidade dessa função, se vale apenas para tornar mais idiomatico
// fn mask(value: u32, offset: u32, n_bits: u32) -> u32 {
//     value >> offset & ((2u32).pow(n_bits as u32) - 1)
// }
