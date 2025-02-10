use std::hash::{DefaultHasher, Hash, Hasher};

/// BloomFilter gives a FIRM no or a PROBABLY yes using probabilistic data structure.
struct SimpleBloomFilter {
    bit_set: Vec<bool>,
    /// Number of hash functions used to determine the bit to check for key existence.
    /// https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions.
    num_probes: u32,
}

impl SimpleBloomFilter {
    pub fn new(size: usize, num_probes: u32) -> Self {
        Self {
            bit_set: vec![false; size],
            num_probes,
        }
    }

    pub fn add_key(&mut self, key: &[u8]) {
        let indices = self.hash_key(key);
        for index in indices {
            self.bit_set[index] = true;
        }
    }

    pub fn has_key(&self, key: &[u8]) -> bool {
        let indices = self.hash_key(key);
        for index in indices {
            if !self.bit_set[index] {
                return false;
            }
        }
        true // if all bits are 1, key may be in set (false positive possible)
    }

    pub fn hash_key(&self, key: &[u8]) -> Vec<usize> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let size = self.bit_set.len() as u64;
        let mut result = Vec::with_capacity(self.num_probes as usize);
        // double hashing scheme to generate multiple unique indices from a single hash value.
        let h1 = (hash << 32) >> 32;
        let h2 = hash >> 32;
        for i in 0..self.num_probes {
            let index = ((h1.wrapping_add(i as u64 * h2)) % size) as usize;
            result.push(index);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_works() {
        let filter_size = 1000;
        let num_probes = 3;
        let mut filter = SimpleBloomFilter::new(filter_size, num_probes);
        filter.add_key(("apple").as_bytes());
        filter.add_key(("banana").as_bytes());
        filter.add_key(("orange").as_bytes());

        assert!(!filter.has_key("grape".as_bytes()));
        assert!(!filter.has_key("mango".as_bytes()));
        assert!(!filter.has_key("kiwi".as_bytes()));
    }
}
