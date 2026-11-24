use std::collections::HashMap;

struct Node {
    block_addr: u32,
    prev: Option<usize>,
    next: Option<usize>,
}

/// Cache totalmente associativa com substituição LRU em tempo O(1) amortizado.
///
/// Utiliza uma lista duplamente encadeada baseada em arena (`Vec<Node>`) combinada
/// com um `HashMap<u32, usize>` que mapeia `block_addr -> índice no Vec`:
/// - `head` aponta para o bloco mais recentemente usado (MRU).
/// - `tail` aponta para o bloco menos recentemente usado (LRU, próximo a ser expulso).
pub struct FullyAssociativeLru {
    capacity: usize,
    nodes: Vec<Node>,
    map: HashMap<u32, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl FullyAssociativeLru {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            nodes: Vec::new(),
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    /// Registra um acesso ao bloco `block_addr`.
    /// Retorna `true` em caso de Hit e `false` em caso de Miss.
    pub fn access(&mut self, block_addr: u32) -> bool {
        if self.capacity == 0 {
            return false;
        }

        // 1. Hit: bloco já está presente -> promove para MRU (head)
        if let Some(&idx) = self.map.get(&block_addr) {
            self.move_to_front(idx);
            return true;
        }

        // 2. Miss com espaço livre: aloca novo nó na arena e insere no início (head)
        if self.nodes.len() < self.capacity {
            let idx = self.nodes.len();
            let old_head = self.head;

            self.nodes.push(Node {
                block_addr,
                prev: None,
                next: old_head,
            });

            if let Some(h) = old_head {
                self.nodes[h].prev = Some(idx);
            } else {
                // Primeiro elemento inserido: também é o tail
                self.tail = Some(idx);
            }

            self.head = Some(idx);
            self.map.insert(block_addr, idx);
            return false;
        }

        // 3. Miss com cache cheia: reaproveita o slot do nó LRU (tail)
        let victim_idx = self.tail.expect("cache cheia sempre possui tail");
        let old_block = self.nodes[victim_idx].block_addr;

        self.map.remove(&old_block);
        self.nodes[victim_idx].block_addr = block_addr;
        self.map.insert(block_addr, victim_idx);
        self.move_to_front(victim_idx);

        false
    }

    /// Desconecta o nó `idx` de sua posição atual e o move para `head` (MRU).
    fn move_to_front(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return;
        }

        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        if let Some(p) = prev {
            self.nodes[p].next = next;
        }

        if let Some(n) = next {
            self.nodes[n].prev = prev;
        } else {
            // `idx` não tinha próximo, portanto era o `tail`
            self.tail = prev;
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        }

        self.head = Some(idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_one() {
        let mut lru = FullyAssociativeLru::new(1);

        assert!(!lru.access(10)); // Miss -> [10]
        assert!(lru.access(10)); // Hit  -> [10]
        assert!(!lru.access(20)); // Miss -> expulsa 10, fica [20]
        assert!(lru.access(20)); // Hit  -> [20]
        assert!(!lru.access(10)); // Miss -> 10 foi expulso
    }

    #[test]
    fn test_lru_eviction_order() {
        let mut lru = FullyAssociativeLru::new(3);

        // Preenche: ordem MRU -> LRU fica [3, 2, 1]
        assert!(!lru.access(1));
        assert!(!lru.access(2));
        assert!(!lru.access(3));

        // Hit no tail (1): promove para head -> [1, 3, 2]
        assert!(lru.access(1));
        // Hit no meio (3): promove para head -> [3, 1, 2]
        assert!(lru.access(3));

        // Novo bloco (4): deve expulsar o LRU atual (2) -> [4, 3, 1]
        assert!(!lru.access(4));
        assert!(lru.access(1)); // ainda está -> [1, 4, 3]
        assert!(lru.access(3)); // ainda está -> [3, 1, 4]
        assert!(lru.access(4)); // ainda está -> [4, 3, 1]
        assert!(!lru.access(2)); // 2 havia sido expulso -> expulsa 1, fica [2, 4, 3]
        assert!(!lru.access(1)); // 1 foi expulso agora
    }

    #[test]
    fn test_repeated_hits_preserve_consistency() {
        let mut lru = FullyAssociativeLru::new(2);

        assert!(!lru.access(100)); // [100]
        assert!(!lru.access(200)); // [200, 100]
        assert!(lru.access(200)); // hit no head repetido -> [200, 100]
        assert!(lru.access(200)); // [200, 100]
        assert!(!lru.access(300)); // expulsa 100 -> [300, 200]
        assert!(lru.access(200)); // 200 continua
        assert!(!lru.access(100)); // 100 saiu
    }
}
